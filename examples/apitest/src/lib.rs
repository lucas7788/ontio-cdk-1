#![cfg_attr(not(feature="mock"), no_std)]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{runtime, console};
use ostd::abi::{Sink, Source, Decoder};
use ostd::types::H160;
use ostd::str;

#[ostd::abi_codegen::contract]
pub trait ApiTest {
    fn timestamp(&self) -> u64;
    fn blockheight(&self) -> u32;
    fn selfaddress(&self) -> Address;
    fn calleraddress(&self) -> Address;
    fn entry_address(&self) -> Address;
    fn contract_debug(&self, content:&str) ->();
//    fn contract_delete(&self) -> ();
    fn checkwitness(&self, addr: &Address) -> bool;
    fn get_current_blockhash(&self) -> H160;
    fn get_current_txhash(&self) -> H160;
    fn call_name(&self, contract_address:&Address) -> String;
    fn call_balance_of(&self, contract_address:&Address, addr:&Address) -> U256;
    fn call_transfer(&self, contract_address:&Address, from: &Address, to:&Address, amount:U256) -> bool;
    fn call_native_transfer(&self, contract_address:&Address, vesion:u8, from: &Address, to:&Address, amount:U256) -> bool;
}

pub(crate) struct ApiTestInstance;

impl ApiTest for ApiTestInstance {
    fn timestamp(&self) -> u64 {
        runtime::timestamp()
    }
    fn blockheight(&self) -> u32 {
        runtime::block_height()
    }
    fn selfaddress(&self) -> Address {
        runtime::address()
    }
    fn calleraddress(&self) -> Address {
        runtime::caller()
    }
    fn entry_address(&self) -> Address {
        runtime::entry_address()
    }
    fn contract_debug(&self, content:&str) {
        console::debug(content);
    }
//    fn contract_delete(&self) -> () {
//        runtime::contract_delete();
//    }
    fn checkwitness(&self, addr: &Address) -> bool {
        let b = runtime::check_witness(addr);
        if b {
            runtime::notify("success".as_bytes());
            true
        } else {
            runtime::notify("failed".as_bytes());
            false
        }
    }
    fn get_current_blockhash(&self) -> H160 {
        let mut temp:[u8;20] = [0;20];
        let mut blockhash = H160::new(temp);
        runtime::current_blockhash(&blockhash);
        blockhash
    }
    fn get_current_txhash(&self) -> H160 {
        let mut temp:[u8;20] = [0;20];
        let mut txhash = H160::new(temp);
        runtime::current_txhash(&txhash);
        txhash
    }
    fn call_name(&self, contract_address:&Address) -> String {
        let mut sink = Sink::new(16);
        sink.write("name".to_string());
        console::debug(&format!("{:?}", contract_address));
        let res = runtime::call_contract(contract_address, sink.into().as_slice()).unwrap();
        let s = str::from_utf8(res.as_slice()).unwrap();
        console::debug(s);
        let mut source = Source::new(res);
        source.read().unwrap()
    }
    fn call_balance_of(&self, contract_address:&Address, addr:&Address) -> U256 {
        let mut sink = Sink::new(16);
        sink.write(("balance_of".to_string(), addr));
        let res = runtime::call_contract(contract_address, sink.into().as_slice());
        if res.is_some() {
            let temp = res.unwrap();
            let mut source =Source::new(temp);
            U256::decode(&mut source).unwrap()
        } else {
            U256::zero()
        }
    }
    fn call_transfer(&self, contract_address:&Address, from: &Address, to:&Address, amount:U256) -> bool {
        let mut sink = Sink::new(16);
        sink.write(("transfer".to_string(),from, to, amount));
        let res = runtime::call_contract(contract_address,sink.into().as_slice());
        if res.is_some() {
            true
        } else {
            false
        }
    }
    fn call_native_transfer(&self, contract_address:&Address, version:u8, from: &Address, to:&Address, amount:U256) -> bool {
        let mut sink = Sink::new(16);
        sink.write((version, "transfer".to_string()));
        //1+1+20+20+32
        sink.write(74u32);
        //transfer length
        sink.write(1u32);
        //state length
        sink.write(1u32);
        sink.write((from, to, amount));
        let res = runtime::call_contract(contract_address,sink.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            runtime::notify("true".as_bytes());
            let s = str::from_utf8(data.as_slice()).unwrap();
            console::debug(s);
            return true;
        } else {
            runtime::notify("false".as_bytes());
            false
        }
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  ApiTestDispatcher::new(ApiTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;