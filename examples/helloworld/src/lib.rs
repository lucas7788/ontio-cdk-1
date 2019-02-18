#![cfg_attr(not(feature="mock"), no_std)]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::runtime;

#[ostd::abi_codegen::contract]
pub trait HelloWorld {
    fn hello(&self) -> String;
}

pub(crate) struct HelloWorldInstance;

impl HelloWorld for HelloWorldInstance {
    fn hello(&self) -> String {
        "hello world".to_string()
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  HelloWorldDispatcher::new(HelloWorldInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;

