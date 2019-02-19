#![no_std]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::{Sink, Source};
use ostd::{database, runtime};

fn hello() -> String {
    "hello world".to_string()
}

fn hello2(data: &str) -> String {
    format!("hello {}", data)
}

fn save(key:&str, value:&str) -> bool {
    database::put(key, value);
    true
}

fn get(key:&str) -> String {
    let value:String = database::get(key).unwrap_or_default();
    value
}

#[no_mangle]
pub fn invoke() {
    let mut source = Source::new(runtime::input());
    let action = source.read::<String>().unwrap();
    let mut sink = Sink::new(12);
    match action.as_str() {
        "hello" => sink.write(hello()),
        "hello2" => {
            let data :String= source.read().unwrap_or_default();
            sink.write(hello2(&data));
        },
        "save" => {
            let key:String = source.read().unwrap_or_default();
            let value:String = source.read().unwrap_or_default();
            sink.write(save(&key, &value));
        },
        "get" => {
            let key:String = source.read().unwrap_or_default();
            sink.write(get(&key));
        }
        _ => panic!("unsupported action!"),
    }
    runtime::ret(&sink.into())
}


