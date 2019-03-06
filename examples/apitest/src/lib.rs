#![cfg_attr(not(feature="mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::{runtime, console};
use ostd::abi::{Sink, Source, Decoder};
use ostd::types::H160;
use ostd::contract::ont;
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
    fn check_witness(&self, addr: &Address) -> bool;
    fn get_current_blockhash(&self) -> H160;
    fn get_current_txhash(&self) -> H160;
    fn call_name(&self, contract_address:&Address) -> String;
    fn call_balance_of(&self, contract_address:&Address, addr:&Address) -> U256;
    fn call_transfer(&self, contract_address:&Address, from: &Address, to:&Address, amount:U256) -> bool;
    fn call_native_transfer(&self, contract_address:&Address, vesion:u8, from: &Address, to:&Address, amount:U256) -> bool;
    fn call_neovm_transfer(&self, contract_address:&Address, from:&Address, to:&Address, amount:U256) -> bool;
    fn call_native_transfer2(&self, version:u8, from: &Address, to:&Address, amount:U256) -> bool;
    fn call_native_balance_of(&self,version:u8, address:&Address) -> U256;
    fn call_native_approve(&self, version:u8, from: &Address, to:&Address, amount:U256) -> bool;
    fn call_native_allowance(&self, version:u8, from: &Address, to:&Address) -> U256;
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
    fn check_witness(&self, addr: &Address) -> bool {
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
        let temp:[u8;20] = [0;20];
        let blockhash = H160::new(temp);
        runtime::current_blockhash(&blockhash);
        blockhash
    }
    fn get_current_txhash(&self) -> H160 {
        let temp:[u8;20] = [0;20];
        let txhash = H160::new(temp);
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
    fn call_neovm_transfer(&self, contract_address:&Address, from:&Address, to:&Address, amount:U256) -> bool {
        console::debug("contract in 111111111111");
        let mut sink = Sink::new(16);
        sink.write(u256_to_native_bytes(amount));
        console::debug("contract in 2222222222");
        sink.write_varuint(20);
        sink.write(to);
        sink.write_varuint(20);
        sink.write(from);
        sink.write(83u8);
        sink.write(193u8);
        sink.write("transfer".to_string());
        sink.write(103u8);
        sink.write(contract_address);
        let res = runtime::call_contract(contract_address,sink.into().as_slice());
        if res.is_some() {
            console::debug("contract in 3333333333333333333");
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
    fn call_native_transfer(&self, contract_address:&Address, version:u8, from: &Address, to:&Address, amount:U256) -> bool {

        let mut sink = Sink::new(16);
        //state length
//        sink.write_varuint(1);
//        sink.write_varuint(1);
//        sink.write_varuint(20);
//        sink.write(from);
//        sink.write_varuint(20);
//        sink.write(to);
//        sink.write(u256_to_native_bytes(amount));

        sink.write_native_varuint(1);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(u256_to_native_bytes(amount));


        let mut sink2 = Sink::new(16);
        sink2.write(version);
        sink2.write("transfer".to_string());
        sink2.write(sink.into());

        let res = runtime::call_contract(contract_address,sink2.into().as_slice());
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
    fn call_native_transfer2(&self, version:u8, from: &Address, to:&Address, amount:U256) -> bool {
        ont::transfer(version, from, to, amount)
    }
    fn call_native_approve(&self, version:u8, from: &Address, to:&Address, amount:U256) -> bool {
        ont::approve(version, from, to, amount)
    }
    fn call_native_allowance(&self, version:u8, from: &Address, to:&Address) -> U256 {
        ont::allowance(version, from, to)
    }
    fn call_native_balance_of(&self,version:u8, address:&Address) -> U256 {
        ont::balance_of(version, address)
    }
}

fn u256_to_native_bytes(data: U256) -> Vec<u8> {
    let mut res:Vec<u8> = Vec::new();
    if data.is_zero() {
        res.push(0);
        return res;
    }
    let mut temp = [0u8;32];
    data.to_big_endian(&mut temp);
    let mut f = false;
    for i in temp.iter() {
        if res.len() ==0 && *i>240u8 {
            f = true;
        }
        if res.len()!=0 || *i != 0u8 {
            res.push(*i);
        }
    }
    res.reverse();
    if f {
        res.push(0);
    }
    res
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  ApiTestDispatcher::new(ApiTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;