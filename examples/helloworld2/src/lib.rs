#![no_std]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::{Sink, Source};
use ostd::runtime;

#[no_mangle]
pub fn invoke() {
    let mut source = Source::new(runtime::input());
    let action = source.read::<String>().unwrap();
    let mut sink = Sink::new(12);
    match action.as_str() {
        "hello" => sink.write("hello world".to_string()),
        _ => panic!("unsupported action!"),
    }

    runtime::ret(&sink.into())
}

