#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Decoder, Encoder, EventBuilder, Sink, Source};
use ostd::macros::base58;
use ostd::prelude::*;
use ostd::prelude::{String, Vec};
use ostd::runtime::check_witness;
use ostd::types::{Address, H256, U128};
use ostd::{contract, database, runtime};

const ADMIN: Address = ostd::macros::base58!("AXdmdzbyf3WZKQzRtrNQwAR91ZxMUfhXkt");
const KEY_CF: &str = "01";
const KEY_DR: &str = "02";

#[derive(Encoder, Decoder)]
struct CrowdFunding {
    user: Address,
    claim_ty: String,
    count: U128,
    price: U128,
    expire: U128,
    pay_condition: U128,
}

fn new_claim_collection(
    user: Address, claim_ty: String, count: U128, price: U128, expire: U128, pay_condition: U128,
) -> bool {
    assert!(runtime::check_witness(&user));
    let amt = count * price;
    assert!(contract::ong::transfer(&user, &ADMIN, amt));
    let cf = CrowdFunding { user, claim_ty, count, price, expire, pay_condition };
    let tx_hash = runtime::current_txhash();
    database::put(utils::gen_cf_key(&tx_hash), cf);
    EventBuilder::new().string("new_claim_collection").h256(&tx_hash).address(&user).notify();
    return true;
}

fn get_crowd_funding_by_id(id: &H256) -> Option<CrowdFunding> {
    return database::get::<_, CrowdFunding>(utils::gen_cf_key(id));
}

fn collection_complete(
    id: &H256, data_set_hash: &H256, collected_count: U128, providers: Vec<&Address>,
) -> bool {
    assert!(runtime::check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdFunding>(utils::gen_cf_key(id)) {
        if providers.len() > cf.count as usize || collected_count > cf.count as u128 {
            return false;
        }
        if collected_count < cf.pay_condition {
            return false;
        }
        for provider in providers {
            assert!(contract::ong::transfer(&ADMIN, provider, cf.price));
        }
        let balance = (cf.count - collected_count) * cf.price;
        assert!(contract::ong::transfer(&ADMIN, &cf.user, balance));
        EventBuilder::new().string("collection_complete").h256(data_set_hash).notify();
    }
    false
}

fn pay_to_providers(id: &H256, providers: Vec<&Address>) -> bool {
    assert!(runtime::check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdFunding>(id) {
        for provider in providers {
            assert!(contract::ong::transfer(&ADMIN, provider, cf.price));
        }
    }
    false
}

fn collection_abort(id: &H256) -> bool {
    assert!(check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdFunding>(id) {
        let amt = cf.price * cf.count;
        assert!(contract::ong::transfer(&ADMIN, &cf.user, amt));
        EventBuilder::new().string("collection_abort").number(amt).notify();
        return true;
    }
    false
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "new_claim_collection" => {
            let (user, claim_ty, count, price, expire, payCondition) = source.read().unwrap();
            sink.write(new_claim_collection(user, claim_ty, count, price, expire, payCondition));
        }
        "get_crowd_funding_by_id" => {
            let id = source.read().unwrap();
            if let Some(res) = get_crowd_funding_by_id(id) {
                sink.write(res);
            }
        }
        "collection_complete" => {
            let (id, data_set_hash, collected_count, providers) = source.read().unwrap();
            sink.write(collection_complete(id, data_set_hash, collected_count, providers));
        }
        "pay_to_providers" => {
            let id = source.read().unwrap();
            sink.write(collection_abort(id));
        }
        "collection_abort" => {
            let id = source.read().unwrap();
            sink.write(collection_abort(id));
        }
        _ => panic!("unsupported action!"),
    }
    runtime::ret(sink.bytes())
}

mod utils {
    use super::*;
    pub fn gen_cf_key(id: &H256) -> Vec<u8> {
        [KEY_CF.as_bytes(), id.as_ref()].concat()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
