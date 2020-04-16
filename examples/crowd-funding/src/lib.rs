#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Decoder, Encoder, EventBuilder, Sink, Source};
use ostd::contract::ong::allowance;
use ostd::database::get;
use ostd::macros::base58;
use ostd::prelude::*;
use ostd::prelude::{String, Vec};
use ostd::runtime::check_witness;
use ostd::types::{Address, H256, U128};
use ostd::{contract, database, runtime};

const ADMIN: Address = ostd::macros::base58!("AXdmdzbyf3WZKQzRtrNQwAR91ZxMUfhXkt");
const KEY_CF: &str = "01";
const KEY_ALL_CROWD_SOURCING: &str = "02";
const KEY_PLATFORM_FEE_RATE: &str = "03";

#[derive(Encoder, Decoder)]
struct CrowdSourcing {
    user: Address,
    data_ty: String,
    count: U128,
    price: U128,
    expire: U128,
    minimal_count: U128,
    platform_fee_rate: U128,
}

fn update_platform_fee_rate(fee: U128) -> bool {
    assert!(fee >= 0 && fee <= 100);
    assert!(runtime::check_witness(&ADMIN));
    database::put(KEY_PLATFORM_FEE_RATE, fee);
    true
}

fn get_platform_fee_rate() -> U128 {
    if let Some(fee) = database::get::<_, U128>(KEY_PLATFORM_FEE_RATE) {
        return fee;
    } else {
        0
    }
}

fn new_crowd_sourcing(
    user: Address, data_ty: String, count: U128, price: U128, expire: U128, minimal_count: U128,
) -> bool {
    assert!(runtime::check_witness(&user));
    let platform_fee_rate = get_platform_fee_rate();
    let amt = count * price + count * price * platform_fee_rate / 100;
    let self_address = runtime::address();
    assert!(contract::ong::transfer(&user, &self_address, amt));
    let cf = CrowdSourcing {
        user,
        data_ty: data_ty.clone(),
        count,
        price,
        expire,
        minimal_count,
        platform_fee_rate,
    };
    let tx_hash = runtime::current_txhash();
    database::put(utils::gen_cf_key(&tx_hash), &cf);
    if let Some(mut all) = get_all_crowd_sourcing() {
        all.push(cf);
        database::put(utils::gen_cf_key(&tx_hash), all);
    } else {
        database::put(utils::gen_cf_key(&tx_hash), vec![cf]);
    }
    EventBuilder::new()
        .string("new_claim_collection")
        .address(&user)
        .string(&data_ty)
        .number(count)
        .number(price)
        .number(expire)
        .number(minimal_count)
        .number(platform_fee_rate)
        .h256(&tx_hash)
        .notify();
    return true;
}

fn get_crowd_sourcing_by_id(id: &H256) -> Option<CrowdSourcing> {
    return database::get::<_, CrowdSourcing>(utils::gen_cf_key(id));
}

fn get_all_crowd_sourcing() -> Option<Vec<CrowdSourcing>> {
    return database::get::<_, Vec<CrowdSourcing>>(KEY_ALL_CROWD_SOURCING);
}

fn crowd_sourcing_complete(
    id: &H256, data_set_hash: &H256, collected_count: U128, providers: Vec<&Address>,
) -> bool {
    assert!(runtime::check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdSourcing>(utils::gen_cf_key(id)) {
        if providers.len() > cf.count as usize || collected_count > cf.count as u128 {
            return false;
        }
        if collected_count < cf.minimal_count {
            return false;
        }
        let self_address = runtime::address();
        for provider in providers {
            assert!(contract::ong::transfer(&self_address, provider, cf.price));
        }
        let fee = collected_count * cf.platform_fee_rate / 100;
        assert!(contract::ong::transfer(&self_address, &ADMIN, fee));
        let balance = (cf.count - collected_count) * cf.price - fee;
        assert!(contract::ong::transfer(&self_address, &cf.user, balance));
        EventBuilder::new().string("collection_complete").h256(data_set_hash).notify();
    }
    false
}

fn pay_to_providers(id: &H256, providers: Vec<&Address>) -> bool {
    assert!(runtime::check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdSourcing>(id) {
        for provider in providers {
            assert!(contract::ong::transfer(&ADMIN, provider, cf.price));
        }
    }
    false
}

fn crowd_sourcing_abort(id: &H256) -> bool {
    assert!(check_witness(&ADMIN));
    if let Some(cf) = database::get::<_, CrowdSourcing>(id) {
        let amt = cf.price * cf.count + cf.price * cf.count * cf.platform_fee_rate;
        assert!(contract::ong::transfer(&ADMIN, &cf.user, amt));
        EventBuilder::new().string("collection_abort").number(amt).notify();
        return true;
    }
    false
}

fn migrate(
    code: &[u8], vm_type: u32, name: &str, version: &str, author: &str, email: &str, desc: &str,
) -> bool {
    let addr = runtime::contract_migrate(code, vm_type, name, version, author, email, desc);
    assert_ne!(addr, Address::zero());
    true
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "update_platform_fee_rate" => {
            let platform_fee_rate = source.read().unwrap();
            sink.write(update_platform_fee_rate(platform_fee_rate));
        }
        "get_platform_fee_rate" => {
            sink.write(get_platform_fee_rate());
        }
        "new_crowd_sourcing" => {
            let (user, claim_ty, count, price, expire, minimal_count) = source.read().unwrap();
            sink.write(new_crowd_sourcing(user, claim_ty, count, price, expire, minimal_count));
        }
        "get_crowd_sourcing_by_id" => {
            let id = source.read().unwrap();
            if let Some(res) = get_crowd_sourcing_by_id(id) {
                sink.write(res);
            }
        }
        "get_all_crowd_sourcing" => {
            if let Some(res) = get_all_crowd_sourcing() {
                sink.write(res);
            }
        }
        "crowd_sourcing_complete" => {
            let (id, data_set_hash, collected_count, providers) = source.read().unwrap();
            sink.write(crowd_sourcing_complete(id, data_set_hash, collected_count, providers));
        }
        "pay_to_providers" => {
            let id = source.read().unwrap();
            sink.write(crowd_sourcing_abort(id));
        }
        "crowd_sourcing_abort" => {
            let id = source.read().unwrap();
            sink.write(crowd_sourcing_abort(id));
        }
        "migrate" => {
            let (code, vm_type, name, version, author, email, desc) = source.read().unwrap();
            sink.write(migrate(code, vm_type, name, version, author, email, desc))
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
