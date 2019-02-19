#![cfg_attr(not(feature="mock"), no_std)]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{runtime,database};

#[ostd::abi_codegen::contract]
pub trait HelloWorld {
    fn hello(&self) -> String;
    fn hello2(&self, data:&str) -> String;
    fn save(&self, key:&str, value:&str) -> bool;
    fn get(&self, key:&str)->String;
}

pub(crate) struct HelloWorldInstance;

impl HelloWorld for HelloWorldInstance {
    fn hello(&self) -> String {
        "hello world".to_string()
    }
    fn hello2(&self, data:&str) -> String {
        format!("hello {}", data)
    }
    fn save(&self, key:&str, value:&str) -> bool {
        database::put(key, value);
        true
    }
    fn get(&self, key:&str)->String {
        database::get(key).unwrap_or_default()
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  HelloWorldDispatcher::new(HelloWorldInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;

