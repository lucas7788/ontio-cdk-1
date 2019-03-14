#![cfg_attr(not(feature="mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use ostd::prelude::*;
use ostd::{database, runtime};
use ostd::abi::{Sink, Source};
const _ADDR_EMPTY: Address = ostd::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");

const KEY_TOTAL_SUPPLY: &'static str = "total_supply";
const TOTAL_SUPPLY: u64 = 1000000000;
const KEY_BALANCE : &'static str = "b";
const KEY_APPROVE : &'static str = "a";

fn initialize(owner:&Address) -> bool {
    if database::get::<_, u64>(KEY_TOTAL_SUPPLY).is_some() {
        return false
    }
    database::put(KEY_TOTAL_SUPPLY,TOTAL_SUPPLY);
    database::put(&utils::gen_balance_key(owner), TOTAL_SUPPLY);
    true
}

fn name() ->String {
    "wasm_token".to_string()
}

fn balance_of(owner: &Address) -> u64 {
    database::get(&utils::gen_balance_key(owner)).unwrap_or(0u64)
}

fn transfer(from: &Address, to: &Address, amount: u64) -> bool {
    if runtime::check_witness(from) == false {
        return false;
    }
    let mut frmbal = balance_of(from);
    let mut tobal = balance_of(to);
    if amount == 0u64 || frmbal < amount {
        false
    } else {
        frmbal = frmbal - amount;
        tobal = tobal + amount;
        database::put(&utils::gen_balance_key(from), &frmbal);
        database::put(&utils::gen_balance_key(to), &tobal);
        true
    }
}
fn transfer_multi(states:&[(Address, Address, u64)]) -> bool {
    if states.is_empty() {
        return false;
    }
    for state in states.iter() {
        if transfer(&state.0, &state.1, state.2) == false {
            panic!("transfer failed, from:{}, to:{}, amount:{}", state.0, state.1, state.2);
        }
    }
    true
}

fn approve(approves: &Address, receiver: &Address, amount: u64) -> bool {
    if runtime::check_witness(approves) == false {
        return false;
    }
    let apprbal = balance_of(approves);
    if apprbal < amount {
        return false;
    } else {
        database::put(&utils::gen_approve_key(approves, receiver), amount);
        true
    }
}
fn transfer_from(receiver: &Address, approves: &Address, amount: u64) -> bool {
    if runtime::check_witness(receiver) == false {
        return false;
    }
    let mut allow = allowance(approves, receiver);
    if allow < amount {
        return false;
    }
    let mut approbal = balance_of(approves);
    if approbal < amount {
        return false;
    }
    let mut receivbal = balance_of(receiver);
    receivbal = receivbal + amount;
    approbal = approbal - amount;
    allow = allow - amount;
    database::put(utils::gen_approve_key(approves, receiver), allow);
    database::put(utils::gen_balance_key(approves), approbal);
    database::put(utils::gen_balance_key(receiver), receivbal);
    true
}
fn allowance(approves: &Address, receiver: &Address) -> u64 {
    database::get(&utils::gen_approve_key(approves, receiver)).unwrap_or(0u64)
}
fn total_supply() -> u64 {
    database::get(KEY_TOTAL_SUPPLY).unwrap_or(064)
}

#[no_mangle]
pub fn invoke() {
    let mut source = Source::new(runtime::input());
    let action = source.read::<String>().unwrap();
    let mut sink = Sink::new(12);
    match action.as_str() {
        "initialize" => {
            let owner = source.read().unwrap();
            sink.write(initialize(&owner));
        }
        "balance_of" => {
            let addr = source.read().unwrap();
            sink.write(balance_of(&addr));
        },
        "transfer" => {
            let from = source.read().unwrap();
            let to = source.read().unwrap();
            let amount = source.read().unwrap();
            sink.write(transfer(&from, &to, amount));
        },
        _ => panic!("unsupported action!"),
    }
    runtime::ret(&sink.into())
}

mod utils {
    use super::*;
    pub fn gen_balance_key(addr: &Address) -> Vec<u8> {
        [KEY_BALANCE.as_bytes(), addr.as_ref()].concat()
    }
    pub fn gen_approve_key(approves:&Address, receiver:&Address) -> Vec<u8> {
        let mut key:Vec<u8> = KEY_APPROVE.as_bytes().to_vec();
        key.extend_from_slice(approves.as_ref());
        key.extend_from_slice(receiver.as_ref());
        key
    }
}

#[cfg(test)]
mod test;