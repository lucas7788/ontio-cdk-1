#![cfg_attr(not(feature="mock"), no_std)]
extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::runtime;

#[ostd::abi_codegen::contract]
pub trait ApiTest {
    fn timestamp(&self) -> u64;
    fn blockheight(&self) -> u64;
    fn selfaddress(&self) -> String;
    fn calleraddress(&self) -> String;
    fn checkwitness(&self, addr: &Address) -> bool;
    fn get_current_block_hash(&self) -> String;
    fn get_current_tx_hash(&self) -> String;
}

pub(crate) struct ApiTestInstance;

impl ApiTest for ApiTestInstance {
    fn timestamp(&self) -> u64 {
        runtime::timestamp()
    }
    fn blockheight(&self) -> u64 {
        runtime::block_height()
    }
    fn selfaddress(&self) -> String {
        let addr = runtime::address();
        format!("{:?}", addr)
    }
    fn calleraddress(&self) -> String {
        let addr = runtime::caller();
        format!("{:?}", addr)
    }
    fn checkwitness(&self, addr: &Address) -> bool {
        runtime::check_witness(addr)
    }
    fn get_current_block_hash(&self) -> String {
        let blockhash = runtime::get_current_block_hash();
        format!("{:?}", blockhash)
    }
    fn get_current_tx_hash(&self) -> String {
        let txhash = runtime::get_current_tx_hash();
        format!("{:?}", txhash)
    }
}

#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  ApiTestDispatcher::new(ApiTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[cfg(test)]
mod test;