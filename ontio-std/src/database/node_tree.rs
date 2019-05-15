#![feature(inner_deref)]
use crate::abi::{Decoder, Encoder, Error, Sink, Source};
use crate::cmp::min;
use crate::database::node;
use crate::database::{get, put};
use crate::vec::Vec;

const MAX_PREFIX_LEN: u32 = 10;

#[derive(Debug)]
pub struct NodeTree {
    pub root: Option<node::Node>,
    pub size: u64,
    pub key: Vec<u8>,
}

fn new_leaf_node(key: &Vec<u8>, value: &Vec<u8>) -> node::Node {
    let mut new_key = Vec::with_capacity(key.len());
    new_key.extend(key);
    return node::Node {
        keys: Vec::new(),
        children: Vec::new(),
        children_bytes: Vec::new(),
        prefix: Vec::new(),
        prefix_len: 0,
        size: 0,
        need_flush: true,
        key: new_key,
        key_size: 0,
        value: value.to_owned(),
        node_type: node::LEAF,
    };
}

impl Drop for NodeTree {
    fn drop(&mut self) {
        self.flush();
    }
}

impl Encoder for NodeTree {
    fn encode(&self, sink: &mut Sink) {
        sink.write(&self.key);
        sink.write_u64(self.size);
        if let Some(root) = &self.root {
            root.encode(sink);
        }
    }
}

impl Decoder for NodeTree {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let key = source.read().unwrap();
        let size = source.read().unwrap();
        let root = node::Node::decode(source).unwrap();
        return Ok(NodeTree { root: Some(root), size, key });
    }
}

impl NodeTree {
    pub fn flush(&mut self) {
        let mut root = &mut self.root;
        if let Some(root) = root {
            for (_, item) in root.children.iter_mut().enumerate() {
                item.flush();
            }
            let mut sink = Sink::new(16);
            self.encode(&mut sink);
            put(&self.key, sink.bytes());
        }
    }
    pub fn insert(&mut self, key: &Vec<u8>, value: &Vec<u8>) {
        let key = ensure_null_terminated_key(&mut key.to_vec());
        if self.root.is_none() {
            self.root = Some(new_leaf_node(&key, &value));
            self.size += 1;
            return;
        }
        let mut depth = 0;
        if insert_inner(self.root.as_mut().unwrap(), &key, value, 0) {
            self.size += 1;
        }
    }

    pub fn search(&mut self, key: &Vec<u8>) -> Option<Vec<u8>> {
        let key = ensure_null_terminated_key(&mut key.clone());
        return search_inner(&mut self.root.clone().unwrap(), &key, 0);
    }
}

pub fn open(key: Vec<u8>) -> Result<NodeTree, Error> {
    let tree = get(key.clone());
    if let Some(t) = tree {
        let mut source = Source::new(t);
        return NodeTree::decode(&mut source);
    } else {
        return Ok(new_tree(key));
    }
}

pub fn new_tree(key: Vec<u8>) -> NodeTree {
    return NodeTree { root: None, size: 0, key };
}

fn search_inner(current_node: &mut node::Node, key: &Vec<u8>, mut depth: u32) -> Option<Vec<u8>> {
    if current_node.is_leaf() {
        if current_node.is_match(key) {
            return Some(current_node.clone().value.clone());
        }
        return None;
    }
    if current_node.prefix_mis_match(key, depth) != current_node.prefix_len {
        return None;
    } else {
        depth += current_node.prefix_len;
    }
    if let Some(node) = current_node.find_child(key[depth as usize]) {
        *current_node = node.to_owned();
        depth += 1;
        return search_inner(current_node, key, depth);
    }
    return None;
}

fn insert_inner(
    current_node: &mut node::Node, key: &Vec<u8>, value: &Vec<u8>, mut depth: u32,
) -> bool {
    if current_node.is_leaf() {
        if current_node.is_match(&key) {
            return false;
        }
        let mut new_node4 = node::new_node(node::NODE4);
        let new_leaf_node = new_leaf_node(&key, &value);
        let limit = current_node.longest_common_prefix(&new_leaf_node, depth);
        new_node4.prefix_len = limit;
        let key_temp: Vec<_> = key
            .iter()
            .enumerate()
            .filter(|&(idx, _)| idx as u32 >= depth)
            .map(|(_, &item)| item)
            .collect();
        new_node4.memcpy(&key_temp, min(MAX_PREFIX_LEN, new_node4.prefix_len));
        let mut current_key = current_node.clone().key.clone();
        let k = current_key.get((depth + new_node4.prefix_len) as usize).unwrap_or(&0u8);
        new_node4.add_child(k.to_owned(), &current_node);
        let k = key.get((depth + new_node4.prefix_len) as usize).unwrap_or(&0u8);
        new_node4.add_child(k.to_owned(), &new_leaf_node);
        *current_node = new_node4;
        return true;
    }
    if current_node.prefix_len != 0 {
        let mismatch = current_node.prefix_mis_match(&key, depth);
        if mismatch != current_node.prefix_len {
            let mut new_node4 = node::new_node(node::NODE4);
            new_node4.prefix_len = mismatch;
            new_node4.memcpy(&mut current_node.clone().prefix, mismatch);
            if current_node.clone().prefix_len < MAX_PREFIX_LEN {
                new_node4.add_child(
                    current_node.clone().prefix.get(mismatch as usize).unwrap().to_owned(),
                    &current_node,
                );
                current_node.prefix_len -= (mismatch + 1);
                let current_temp: Vec<u8> = current_node
                    .clone()
                    .prefix
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx >= (mismatch + 1) as usize)
                    .map(|(idx, &item)| item)
                    .collect();
                current_node.memmove(&current_temp, min(current_node.prefix_len, MAX_PREFIX_LEN));
            } else {
                current_node.prefix_len -= (mismatch + 1);
                let min_key = current_node.minimum().unwrap().key.clone();
                new_node4.add_child(
                    min_key.clone().get((depth + mismatch) as usize).unwrap().to_owned(),
                    &current_node,
                );
                let min_key_temp: Vec<u8> = min_key
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx > (depth + mismatch + 1) as usize)
                    .map(|(idx, &item)| item)
                    .collect();
                current_node.memmove(&min_key_temp, min(current_node.prefix_len, MAX_PREFIX_LEN));
            }
            let mut new_leaf_node = new_leaf_node(&key, &value);
            new_node4.add_child(
                key.get((depth + mismatch) as usize).unwrap().to_owned(),
                &new_leaf_node,
            );
            *current_node = new_node4;
            return true;
        }
        depth += current_node.clone().prefix_len;
    }
    let mut next = current_node.find_child_mut(key.get(depth as usize).unwrap().to_owned());
    if let Some(node) = next {
        return insert_inner(node, &key, value, depth + 1);
    } else {
        current_node
            .add_child(key.get(depth as usize).unwrap().to_owned(), &new_leaf_node(&key, &value));
        return true;
    }
}

fn memcpy(dest: &mut Vec<u8>, src: &Vec<u8>, num_bytes: u32) {
    let mut i = 0;
    loop {
        if i >= num_bytes || i >= dest.len() as u32 || i >= src.len() as u32 {
            break;
        }
        dest.insert(i as usize, src.get(i as usize).unwrap().to_owned());
        i += 1;
    }
}

fn memmove(dest: &mut Vec<u8>, src: &Vec<u8>, num_bytes: u32) {
    let mut index = 0;
    loop {
        if index >= num_bytes {
            break;
        }
        dest.insert(index as usize, src.get(index as usize).unwrap().to_owned());
        index += 1;
    }
}

fn ensure_null_terminated_key(key: &mut Vec<u8>) -> Vec<u8> {
    let index = key.iter().position(|&item| item == 0);
    match index {
        Some(ind) => {
            key.push(0u8);
        }
        None => {}
    }
    return key.to_vec();
}

#[test]
fn test2() {
    let mut t = new_tree("key".as_bytes().to_vec());
    let key = "abc".as_bytes().to_vec();
    let value = "value1".as_bytes().to_vec();
    t.insert(&key, &value);
    println!("t1: {:?}", t);
    t.flush();
    let t2 = open("key".as_bytes().to_vec());
    println!("t2: {:?}", t2.unwrap());
    assert_eq!(1, 2);
}

#[test]
fn test() {
    let mut t = new_tree("key".as_bytes().to_vec());
    let key = "abc".as_bytes().to_vec();
    let value = "value1".as_bytes().to_vec();
    t.insert(&key, &value);
    println!("t1: {:?}", t);

    let key = "abc1".as_bytes().to_vec();
    let value = "value2".as_bytes().to_vec();
    t.insert(&key, &value);
    println!("t2: {:?}", t);
    let key = "dabc".as_bytes().to_vec();
    let value = "value3".as_bytes().to_vec();
    t.insert(&key, &value);
    println!("t3: {:?}", t);
    let key = "abcd".as_bytes().to_vec();
    let value = "value4".as_bytes().to_vec();
    t.insert(&key, &value);
    println!("t4: {:?}", t);
    let key = "dbcd".as_bytes().to_vec();
    let value = "value5".as_bytes().to_vec();
    t.insert(&key, &value);

    println!("t5: {:?}", t);

    let res = t.search(&key);
    if res.is_some() {
        println!("res:{:?}", res.unwrap());
    } else {
        println!("ssss");
    }
    assert_eq!(1, 2);
}
