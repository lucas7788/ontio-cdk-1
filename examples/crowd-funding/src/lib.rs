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
const KEY_SUCCESS_PLATFORM_FEE_RATE: &str = "03";
const KEY_FAILED_PLATFORM_FEE_RATE: &str = "04";
const PLATFORM_FEE_RATE_LIMIT: U128 = 10000;

#[derive(Encoder, Decoder)]
struct CrowdSourcing {
    user: Address,
    data_ty: String,
    count: U128,
    price: U128,
    expire: U128,
    minimal_count: U128,
    success_platform_fee_rate: U128,
    failed_platform_fee_rate: U128,
}

fn update_success_platform_fee_rate(fee: U128) -> bool {
    assert!(fee >= 0 && fee <= PLATFORM_FEE_RATE_LIMIT);
    assert!(runtime::check_witness(&ADMIN));
    database::put(KEY_SUCCESS_PLATFORM_FEE_RATE, fee);
    true
}

fn get_success_platform_fee_rate() -> U128 {
    if let Some(fee) = database::get::<_, U128>(KEY_SUCCESS_PLATFORM_FEE_RATE) {
        return fee;
    } else {
        0
    }
}

fn update_failed_platform_fee_rate(fee: U128) -> bool {
    assert!(fee >= 0 && fee <= PLATFORM_FEE_RATE_LIMIT);
    assert!(runtime::check_witness(&ADMIN));
    database::put(KEY_FAILED_PLATFORM_FEE_RATE, fee);
    true
}

fn get_failed_platform_fee_rate() -> U128 {
    if let Some(fee) = database::get::<_, U128>(KEY_FAILED_PLATFORM_FEE_RATE) {
        return fee;
    } else {
        0
    }
}

fn new_crowd_sourcing(
    user: Address, data_ty: String, count: U128, price: U128, expire: U128, minimal_count: U128,
) -> bool {
    let now = runtime::timestamp();
    assert!(expire > now as U128);
    assert!(runtime::check_witness(&user));
    let success_platform_fee_rate = get_success_platform_fee_rate();
    let failed_platform_fee_rate = get_failed_platform_fee_rate();
    let amt = count * price + count * price * success_platform_fee_rate / PLATFORM_FEE_RATE_LIMIT;
    let self_address = runtime::address();
    assert!(contract::ong::transfer(&user, &self_address, amt));
    let cf = CrowdSourcing {
        user,
        data_ty: data_ty.clone(),
        count,
        price,
        expire,
        minimal_count,
        success_platform_fee_rate,
        failed_platform_fee_rate,
    };
    let tx_hash = runtime::current_txhash();
    database::put(utils::gen_cf_key(&tx_hash), &cf);
    let mut all = get_all_crowd_sourcing();
    all.push(tx_hash.clone());
    database::put(KEY_ALL_CROWD_SOURCING, all);
    EventBuilder::new()
        .string("new_crowd_sourcing")
        .address(&user)
        .string(&data_ty)
        .number(count)
        .number(price)
        .number(expire)
        .number(minimal_count)
        .number(success_platform_fee_rate)
        .number(failed_platform_fee_rate)
        .h256(&tx_hash)
        .notify();
    return true;
}

fn get_crowd_sourcing_by_id(id: &H256) -> Option<CrowdSourcing> {
    return database::get::<_, CrowdSourcing>(utils::gen_cf_key(id));
}

fn get_all_crowd_sourcing() -> Vec<H256> {
    return database::get(KEY_ALL_CROWD_SOURCING).unwrap_or_default();
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
        let fee = collected_count * cf.success_platform_fee_rate / PLATFORM_FEE_RATE_LIMIT;
        if fee > 0 {
            assert!(contract::ong::transfer(&self_address, &ADMIN, fee));
        }
        let balance = (cf.count - collected_count) * cf.price - fee;
        if balance > 0 {
            assert!(contract::ong::transfer(&self_address, &cf.user, balance));
        }
        EventBuilder::new()
            .string("crowd_sourcing_complete")
            .h256(id)
            .h256(data_set_hash)
            .number(collected_count)
            .notify();
        return true;
    }
    false
}

fn pay_to_providers(id: &H256, providers: Vec<&Address>) -> bool {
    assert!(runtime::check_witness(&ADMIN));
    let self_address = runtime::address();
    if let Some(cf) = database::get::<_, CrowdSourcing>(id) {
        for provider in providers {
            assert!(contract::ong::transfer(&self_address, provider, cf.price));
        }
    }
    EventBuilder::new().string("pay_to_providers").h256(id).notify();
    true
}

fn crowd_sourcing_abort(id: &H256) -> bool {
    assert!(check_witness(&ADMIN));
    let self_address = runtime::address();
    if let Some(cf) = database::get::<_, CrowdSourcing>(id) {
        let failed_fee_rate = get_failed_platform_fee_rate();
        let failed_fee = cf.count * cf.price * failed_fee_rate;
        assert!(contract::ong::transfer(&self_address, &ADMIN, failed_fee));
        let amt = cf.price * cf.count + cf.price * cf.count * cf.success_platform_fee_rate;
        let balance = amt - failed_fee;
        assert!(contract::ong::transfer(&self_address, &cf.user, balance));
        EventBuilder::new()
            .string("crowd_sourcing_abort")
            .h256(id)
            .address(&cf.user)
            .number(amt)
            .notify();
        return true;
    }
    false
}

fn migrate(
    code: &[u8], vm_type: u32, name: &str, version: &str, author: &str, email: &str, desc: &str,
) -> bool {
    let addr = runtime::contract_migrate(code, vm_type, name, version, author, email, desc);
    assert_ne!(addr, Address::zero());
    let self_address = runtime::address();
    let balance = contract::ong::balance_of(&self_address);
    assert!(contract::ong::transfer(&self_address, &addr, balance));
    true
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "update_success_platform_fee_rate" => {
            let platform_fee_rate = source.read().unwrap();
            sink.write(update_success_platform_fee_rate(platform_fee_rate));
        }
        "get_success_platform_fee_rate" => {
            sink.write(get_success_platform_fee_rate());
        }
        "update_failed_platform_fee_rate" => {
            let platform_fee_rate = source.read().unwrap();
            sink.write(update_failed_platform_fee_rate(platform_fee_rate));
        }
        "get_failed_platform_fee_rate" => {
            sink.write(get_failed_platform_fee_rate());
        }
        "new_crowd_sourcing" => {
            let (user, data_ty, count, price, expire, minimal_count) = source.read().unwrap();
            sink.write(new_crowd_sourcing(user, data_ty, count, price, expire, minimal_count));
        }
        "get_crowd_sourcing_by_id" => {
            let id = source.read().unwrap();
            if let Some(res) = get_crowd_sourcing_by_id(id) {
                sink.write(res);
            }
        }
        "get_all_crowd_sourcing" => {
            sink.write(get_all_crowd_sourcing());
        }
        "crowd_sourcing_complete" => {
            let (id, data_set_hash, collected_count, providers) = source.read().unwrap();
            sink.write(crowd_sourcing_complete(id, data_set_hash, collected_count, providers));
        }
        "pay_to_providers" => {
            let (id, providers) = source.read().unwrap();
            sink.write(pay_to_providers(id, providers));
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
    extern crate ontio_std as ostd;
    use ontio_std::abi::{Decoder, Sink, Source};
    use ostd::contract::contract::Command;
    use ostd::mock::build_runtime;
    use ostd::prelude::*;
    use ostd::types::{Address, H256};
    use std::collections::HashMap;

    #[test]
    fn test_update_success_fee_rate() {
        assert_eq!(crate::get_success_platform_fee_rate(), 0);
        let handle = build_runtime();
        handle.witness(&[crate::ADMIN]);
        assert!(crate::update_success_platform_fee_rate(10));
        assert_eq!(crate::get_success_platform_fee_rate(), 10);
    }

    #[test]
    fn test_update_failed_fee_rate() {
        assert_eq!(crate::get_failed_platform_fee_rate(), 0);
        let handle = build_runtime();
        handle.witness(&[crate::ADMIN]);
        assert!(crate::update_failed_platform_fee_rate(10));
        assert_eq!(crate::get_failed_platform_fee_rate(), 10);
    }

    #[test]
    fn test_new_crowd_sourcing() {
        let handle = build_runtime();
        handle.timestamp(10);
        let contractAddr = Address::repeat_byte(2);
        handle.address(&contractAddr);
        let tx_hash = H256::new([0u8; 32]);
        handle.current_txhash(&tx_hash);
        let user = Address::repeat_byte(1);
        handle.witness(&[crate::ADMIN, user]);
        let user = Address::repeat_byte(1);
        let mut ong_balance_map = HashMap::<Address, U128>::new();
        ong_balance_map.insert(crate::ADMIN.clone(), 10000);
        ong_balance_map.insert(user.clone(), 10000);

        let mut sink = Sink::new(12);

        let call_contract = move |_addr: &Address, _data: &[u8]| -> Option<Vec<u8>> {
            let mut source = Source::new(_data);
            let command = Command::decode(&mut source);
            if command.is_ok() {
                if let Some(command2) = command.ok() {
                    match command2 {
                        Command::Transfer { from, to, value } => {
                            let mut from_ba =
                                ong_balance_map.get(from).map(|val| val.clone()).unwrap();
                            let mut to_ba =
                                ong_balance_map.get(to).map(|val| val.clone()).unwrap_or_default();
                            from_ba -= value;
                            to_ba += value;
                            ong_balance_map.insert(from.clone(), from_ba);
                            ong_balance_map.insert(to.clone(), to_ba);
                            sink.write(true);
                        }
                        Command::BalanceOf { addr } => {
                            let ba = ong_balance_map.get(addr).map(|val| val.clone()).unwrap();
                            sink.write(ba);
                        }
                        _ => {}
                    }
                }
            }
            return Some(sink.bytes().to_vec());
        };

        handle.on_contract_call(call_contract);

        assert!(crate::new_crowd_sourcing(user.clone(), "data_ty".to_string(), 100, 1, 20, 10));
        let all = crate::get_all_crowd_sourcing();
        assert_eq!(all.len(), 1);
        let cf = crate::get_crowd_sourcing_by_id(&tx_hash).unwrap();
        assert_eq!(cf.data_ty, "data_ty".to_string());

        assert!(crate::crowd_sourcing_complete(&tx_hash, &tx_hash, 100, vec![&crate::ADMIN]));
    }
}
