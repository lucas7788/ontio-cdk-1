#![cfg_attr(not(feature="mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{runtime, console, database};
use ostd::abi::{Sink, Source, Decoder};
use ostd::types::{H160,to_neo_bytes};
use ostd::str;
#[ostd::abi_codegen::contract]
pub trait BasicTest {
    fn save(&self, key:&str, value:&str) -> bool;
    fn get(&self, key:&str) -> String;
    fn get_put(&self) -> String;
    fn delete(&self, key:&str) -> bool;
    fn migrate(&self, code: &[u8], vm_type: u32, name:&str, version:&str,author: &str, email:&str,
               desc:&str) -> bool;
}

pub(crate) struct BasicTestInstance;

impl ApiTest for BasicTestInstance {
    fn save(&self, key:&str, value:&str) -> bool {
        database::put(key, value);
        true
    }
    fn get(&self, key:&str) -> String {
        if let Some(res) = database::get(key) {
            console::debug("55555555555555555");
            return res;
        }
        "failed".to_string()
    }

    fn get_put(&self) -> String {
        let mut value = [0u8;100];
        for i in 0..100 {
            value[i] = i as u8;
        }
        database::put("key", value.to_vec());
        database::get("key").unwrap()
    }
    fn delete(&self, key:&str) -> bool {
        database::delete(key);
        true
    }
    fn migrate(&self, code: &[u8], vm_type: u32, name:&str, version:&str,author: &str, email:&str,
               desc:&str) -> bool {
        runtime::contract_migrate(code, vm_type, name, version, author, email, desc);
        true
    }
}


#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  BasicTestDispatcher::new(BasicTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;