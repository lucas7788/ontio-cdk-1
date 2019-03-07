#![cfg_attr(not(feature="mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{runtime, console, database};
use ostd::abi::{Sink, Source, Decoder};
use ostd::types::{H160,to_neo_bytes};
use ostd::contract::ont;
use ostd::str;

#[ostd::abi_codegen::contract]
pub trait ApiTest {
    fn save(&self, key:&str, value:&str) -> bool;
    fn get(&self, key:&str) -> String;
    fn get_put(&self) -> String;
}

pub(crate) struct ApiTestInstance;

impl ApiTest for ApiTestInstance {
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
}


#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  ApiTestDispatcher::new(ApiTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;