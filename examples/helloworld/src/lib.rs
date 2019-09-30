#![no_std]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime;

fn say_hello(msg: &str) -> String {
    return msg.to_string();
}

fn get_u128(msg: U128) -> U128 {
    return msg;
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"hello" => {
            let msg = source.read().unwrap();
            sink.write(say_hello(msg));
        },
        b"getnum" => {
            let msg = source.read().unwrap();
            sink.write(get_u128(msg));
        },
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}

#[test]
fn test_hello() {

    let res = say_hello("");
    assert_eq!(res, "hello world".to_string());
}
