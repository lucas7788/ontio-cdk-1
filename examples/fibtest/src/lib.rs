#![cfg_attr(not(feature="mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;
use ostd::prelude::*;
use ostd::abi::Dispatcher;
use ostd::types::{U256};
use ostd::{runtime,console, database};

#[ostd::abi_codegen::contract]
pub trait FibTest {
    fn compute(&self, u:U256) -> U256;
    fn compute2(&self, u:U256) -> U256;
    fn compute3(&self, u:U256) -> bool;
    fn compute01(&self, u:U256) -> U256;
    fn compute02(&self, u:U256) -> U256;
}

pub(crate) struct FibTestInstance;

impl FibTest for FibTestInstance {
    fn compute02(&self, u:U256) -> U256 {
        let mut sum = 0;
        for i in 0..u.as_u64() {
            sum = sum + i*i;
        }
        U256::from(sum)
    }
    fn compute3(&self, u:U256) -> bool {
        for i in 0..u.as_u64() {
            database::put(format!("{}", i), format!("{}", i));
        }
        true
    }
    fn compute2(&self, u:U256) -> U256 {
        let mut sum = U256::zero();
        for i in 0..u.as_u64() {
            sum = sum + U256::from(i);
        }
        sum
    }
    fn compute(&self, u:U256) -> U256 {
        let mut sum = U256::zero();
        for i in 0..u.as_u64() {
            let temp = U256::from(i);
            sum = sum + temp * temp;
        }
        sum
    }
    fn compute01(&self, u:U256) -> U256 {
        let mut sum = U256::zero();
        for j in 0..10 {
            for i in 0..u.as_u64() {
                let temp = U256::from(i);
                sum = sum + temp * temp;
            }
        }
        sum
    }
}
fn fib(n: u64) -> U256 {
    let mut f0 = U256::zero();
    let mut f1 = U256::one();
    match n {
        0 | 1 => f0, // 0 为无效值，返回 0
        2 => f1,
        _ => {
            fib(n-1) + fib(n-2)
        }
    }
}


#[no_mangle]
pub fn invoke() {
    let mut dispatcher =  FibTestDispatcher::new(FibTestInstance);
    runtime::ret(&dispatcher.dispatch(&runtime::input()));
}

#[test]
fn com() {
    let fib = FibTestInstance;
    let res = fib.compute(U256::from(200000));
    println!("{}", res);
    assert_eq!(false, true);
}

