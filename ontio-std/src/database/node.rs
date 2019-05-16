use crate::abi::Error;
use crate::abi::{Decoder, Encoder, Sink, Source};
use crate::cmp::min;
use crate::database;

pub const NODE4: u8 = 0;
pub const NODE16: u8 = 1;
pub const NODE48: u8 = 2;
pub const NODE256: u8 = 3;
pub const LEAF: u8 = 4;

pub const NODE4MIN: u32 = 2;
pub const NODE4MAX: u32 = 4;

pub const NODE16MIN: u32 = 5;
pub const NODE16MAX: u32 = 16;

pub const NODE48MIN: u32 = 17;
pub const NODE48MAX: u32 = 48;

pub const NODE256MIN: u32 = 49;
pub const NODE256MAX: u32 = 256;

pub const MAX_PREFIX_LEN: u32 = 10;

#[derive(Debug, Clone)]
pub struct Node {
    // Internal Node Attributes
    pub keys: Vec<u8>,
    pub children: Vec<Node>,
    pub children_bytes: Vec<u8>,
    pub prefix: Vec<u8>,
    pub prefix_len: u32,
    pub size: u32,
    pub need_flush: bool,

    // Leaf Node Attributes
    pub key: Vec<u8>,
    pub key_size: u64,
    pub value: Vec<u8>,
    pub node_type: u8,
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut sink = Sink::new(16);
        self.encode(&mut sink);
        database::put(&self.key, sink.bytes());
    }
}

impl Encoder for Node {
    fn encode(&self, sink: &mut Sink) {
        sink.write(&self.keys);
        sink.write(&self.children_bytes);
        sink.write(&self.prefix);
        sink.write(self.prefix_len);
        sink.write(self.size);
        sink.write(&self.key);
        sink.write(self.key_size);
        sink.write(&self.value);
        sink.write(self.node_type);
    }
}

impl Decoder for Node {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let keys = source.read().unwrap();
        let children_bytes = source.read().unwrap();
        let prefix = source.read().unwrap();
        let prefix_len = source.read_u32().unwrap();
        let size = source.read_u32().unwrap();
        let key = source.read().unwrap();
        let key_size = source.read_u64().unwrap();
        let value = source.read().unwrap();
        let node_type = source.read().unwrap();
        return Ok(Node {
            keys,
            children_bytes,
            children: Vec::new(),
            prefix,
            prefix_len,
            size,
            need_flush: false,
            key,
            key_size,
            value,
            node_type,
        });
    }
}

impl Node {
    pub fn flush(&mut self) {
        if self.children.len() != 0 {
            for (i, item) in self.children.iter_mut().enumerate() {
                item.flush();
            }
        }
        if self.need_flush {
            let mut sink = Sink::new(16);
            self.encode(&mut sink);
            database::put(&self.key, sink.bytes());
        }
    }
    pub fn is_leaf(&self) -> bool {
        self.node_type == LEAF
    }

    pub fn is_match(&self, key: &Vec<u8>) -> bool {
        if self.node_type != LEAF {
            return false;
        }
        self.key == key.to_owned()
    }
    pub fn memcpy(&mut self, src: &Vec<u8>, num_bytes: u32) {
        let mut i = 0;
        loop {
            if i >= num_bytes || i >= src.len() as u32 {
                break;
            }
            self.prefix.insert(i as usize, src.get(i as usize).unwrap().to_owned());
            i += 1;
        }
    }

    pub fn memmove(&mut self, src: &Vec<u8>, num_bytes: u32) {
        let mut index = 0;
        loop {
            if index >= num_bytes {
                break;
            }
            self.prefix[index as usize] = src.get(index as usize).unwrap().to_owned();
            index += 1;
        }
    }
    pub fn longest_common_prefix(&self, other: &Node, depth: u32) -> u32 {
        let limit = min(self.key.clone().len(), other.key.clone().len()) - (depth as usize);
        let mut i = 0;
        loop {
            if i >= limit {
                return i as u32;
            }
            if self.key.clone().get(depth as usize + i) != other.key.clone().get(depth as usize + i)
            {
                return i as u32;
            }
            i += 1;
        }
        return i as u32;
    }
    pub fn is_full(&self) -> bool {
        self.size as u32 == self.max_size()
    }
    pub fn max_size(&self) -> u32 {
        match self.node_type {
            NODE4 => {
                return NODE4MAX;
            }
            NODE16 => {
                return NODE16MAX;
            }
            NODE48 => {
                return NODE48MAX;
            }
            NODE256 => {
                return NODE256MAX;
            }
            _ => {
                return 0;
            }
        }
        return 0;
    }
    pub fn minimum(&self) -> Option<Node> {
        match self.node_type {
            LEAF => {
                return Some(self.to_owned());
            }
            NODE4 | NODE16 => {
                let n = self.children.clone().get(0).unwrap().to_owned();
                return Some(n);
            }
            NODE48 => {
                let mut i = 0;
                loop {
                    if let Some(&k) = self.keys.clone().get(i) {
                        if k == 0 {
                            i += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                let children = self.children.clone();
                let child = children.get((self.keys.clone().get(i).unwrap() - 1) as usize).unwrap();
                return child.minimum();
            }
            NODE256 => {
                let mut i = 0;
                loop {
                    if self.children.clone().get(i).is_none() {
                        i += 1;
                    } else {
                        break;
                    }
                }
                return self.children.clone().get(i).unwrap().minimum();
            }
            _ => {}
        }
        return Some(self.to_owned());
    }

    pub fn prefix_mis_match(&self, key: &Vec<u8>, depth: u32) -> u32 {
        let mut index = 0;
        if self.prefix_len > MAX_PREFIX_LEN {
            for (ind, &v) in key.iter().enumerate() {
                if index >= MAX_PREFIX_LEN {
                    break;
                }
                if key.get(depth as usize + ind) != self.prefix.clone().get(ind) {
                    return ind as u32;
                }
                index += 1;
            }
            let min_key = &self.minimum().unwrap().key;
            loop {
                if index >= self.prefix_len {
                    break;
                }
                if key.get((depth + index) as usize) != min_key.get((depth + index) as usize) {
                    return index;
                }
                index += 1;
            }
        } else {
            let prefix = &self.prefix.clone();
            loop {
                if index >= self.prefix_len {
                    break;
                }
                if key.get((depth + index) as usize) != prefix.get(index as usize) {
                    return index;
                }
                index += 1;
            }
        }
        index
    }

    pub fn find_child_mut(&mut self, key: u8) -> Option<&mut Node> {
        match self.node_type {
            NODE4 | NODE16 | NODE48 => {
                let index = self.index(key);
                if index >= 0 {
                    return self.children.get_mut(index as usize);
                }
                return None;
            }
            NODE256 => {
                return self.children.get_mut(key as usize);
            }
            _ => {}
        }
        return None;
    }
    pub fn find_child(&self, key: u8) -> Option<&Node> {
        match self.node_type {
            NODE4 | NODE16 | NODE48 => {
                let index = self.index(key);
                if index >= 0 {
                    return self.children.get(index as usize);
                }
                return None;
            }
            NODE256 => {
                return self.children.get(key as usize);
            }
            _ => {}
        }
        return None;
    }
    pub fn index(&self, key: u8) -> i32 {
        match self.node_type {
            NODE4 => {
                let keys = &self.keys.clone();
                for (ind, &v) in keys.iter().enumerate() {
                    if v == key {
                        return ind as i32;
                    }
                }
                return -1;
            }
            NODE16 => {
                let keys = &self.keys.clone();
                for (ind, &v) in keys.iter().enumerate() {
                    if v >= key {
                        if ind < keys.len() && keys.get(ind).unwrap().to_owned() == key {
                            return ind as i32;
                        }
                    }
                }
                return -1;
            }
            NODE48 => {
                let &index = self.keys.clone().get(key as usize).unwrap();
                if index > 0 {
                    return (index - 1) as i32;
                }
                return -1;
            }
            NODE256 => {
                return key as i32;
            }
            _ => {
                return -1;
            }
        }
        return -1;
    }
    pub fn add_child(&mut self, key: u8, node: &Node) {
        match self.node_type {
            NODE4 => {
                if !self.is_full() {
                    let mut index = 0;
                    for (ind, &v) in self.keys.iter().enumerate() {
                        if key < self.keys[index] {
                            break;
                        }
                        index += 1;
                    }
                    let mut ind = self.size;
                    loop {
                        if ind <= index as u32 {
                            break;
                        }
                        if self.keys.get((ind - 1) as usize).unwrap_or(&0u8).to_owned() > key {
                            self.keys[ind as usize] = self.keys[(ind - 1) as usize];
                            if self.children.len() == 0
                                || self.children.get(ind as usize).is_none()
                                || ind >= (self.children.len() as u32)
                            {
                                self.children.insert(
                                    ind as usize,
                                    self.children[(ind - 1) as usize].clone(),
                                );
                            } else {
                                self.children[ind as usize] =
                                    self.children[(ind - 1) as usize].clone();
                            }
                        }
                        ind -= 1;
                    }
                    if self.keys.len() <= index {
                        self.keys.insert(index, key);
                    } else {
                        self.keys[index as usize] = key;
                    }
                    if self.children.len() == 0
                        || self.children.get(index as usize).is_none()
                        || index >= (self.children.len() as usize)
                    {
                        self.children.insert(index as usize, node.clone());
                    } else {
                        self.children[index as usize] = node.clone();
                    }
                    self.size += 1;
                } else {
                    self.grow();
                    self.add_child(key, node);
                }
            }
            NODE16 => {
                if !self.is_full() {
                    let mut index = 0;
                    for (ind, &v) in self.keys.iter().enumerate() {
                        if v >= key {
                            break;
                        }
                        index += 1;
                    }
                    let mut i = self.size;
                    loop {
                        if i <= index as u32 {
                            break;
                        }
                        if self.keys.get((i - 1) as usize).unwrap_or(&0u8).to_owned() > key {
                            if self.keys.len() <= i as usize {
                                self.keys.insert(i as usize, self.keys.get((i-1) as usize).unwrap_or(&0u8).to_owned());
                            } else {
                                self.keys[i as usize] = self.keys[(i - 1) as usize];
                            }
                            if self.children.len() <= i as usize {
                                self.children.insert(i as usize, self.children[(i - 1) as usize].clone());
                            } else {
                                self.children[i as usize] = self.children[(i - 1) as usize].clone();
                            }
                        }
                        i -= 1;
                    }
                    if self.keys.len() <= index {
                        self.keys.insert(index as usize, key);
                    } else {
                        self.keys[index as usize] = key;
                    }
                    if self.children.len() <= index {
                        self.children.insert(index, node.clone());
                    } else {
                        self.children[index as usize] = node.clone();
                    }
                    self.size += 1;
                } else {
                    self.grow();
                    self.add_child(key, node);
                }
            }
            NODE48 => if !self.is_full() {},
            _ => {}
        }
    }
    pub fn grow(&mut self) {
        match self.node_type {
            NODE4 => {
                self.node_type = NODE16;
            }
            NODE16 => {
                self.node_type = NODE48;
            }
            NODE48 => {
                self.node_type = NODE256;
            }
            NODE256 => {}
            _ => {}
        }
    }
}

pub fn new_node(node_type: u8) -> Node {
    match node_type {
        NODE4 => {
            return Node {
                keys: Vec::new(),
                children: Vec::new(),
                children_bytes: Vec::new(),
                prefix: Vec::with_capacity(MAX_PREFIX_LEN as usize),
                prefix_len: 0,
                size: 0,
                need_flush: true,
                key: Vec::new(),
                key_size: 0,
                value: Vec::new(),
                node_type,
            };
        }
        NODE16 => {
            return Node {
                keys: Vec::new(),
                children: Vec::new(),
                children_bytes: Vec::new(),
                prefix: Vec::with_capacity(MAX_PREFIX_LEN as usize),
                prefix_len: 0,
                size: 0,
                need_flush: true,
                key: Vec::new(),
                key_size: 0,
                value: Vec::new(),
                node_type,
            }
        }
        _ => {}
    }
    return Node {
        keys: Vec::new(),
        children: Vec::new(),
        children_bytes: Vec::new(),
        prefix: Vec::new(),
        prefix_len: 0,
        size: 0,
        need_flush: true,
        key: Vec::new(),
        key_size: 0,
        value: Vec::new(),
        node_type,
    };
}
