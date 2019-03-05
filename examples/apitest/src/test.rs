extern crate hex;
extern crate ontio_std as ostd;
use crate::{ApiTestInstance};
use ostd::types::Address;
use ostd::abi::{Sink, Source};
use ostd::types::U256;
use ostd::vec::Vec;

const _from: Address = ostd::base58!("Ad4pjz2bqep4RhQrUAzMuZJkBC3qJ1tZuT");

const _to: Address = ostd::base58!("AS3SCXw8GKTEeXpdwVw7EcC4rqSebFYpfb");

#[test]
fn initialize() {
    let from = Address::zero();
    let to = Address::zero();
    let amount = U256::from(1);
    let api = ApiTestInstance;
    let mut sink2 = Sink::new(16);
    //state length
    sink2.write(1u32);
    sink2.write((_from, _to, amount));

    let mut sink = Sink::new(16);
    let data = sink2.into();
    sink.write(data.as_slice());

    let data = sink.into();
    println!("{:?}", data);
    println!("{}", hex::encode(data.as_slice()));
    assert_eq!(false, true);
}

fn u256_to_neo_bytes(data: U256) -> Vec<u8> {
    if data.is_zero() {
        return vec![0];
    }
    let mut temp = [0u8;32];
    data.to_little_endian(&mut temp);
    temp.reverse();

    vec![0]
}

#[test]
fn timestamp(){
//    let data = U256::from(255);
    let  data = U256::from_dec_str("90123123981293054321").unwrap();
    let mut temp = [0u8;32];
    data.to_little_endian(&mut temp);
    temp.reverse();
    println!("temp:{}", hex::encode(temp.to_vec()));
    let mut res:Vec<u8> = Vec::new();
    if data.is_zero() {
        res.push(0);
        println!("res:{}", hex::encode(res));
        assert_eq!(false, true);
        return;
    }
    let mut f = false;
    for i in temp.iter() {
        if res.len() ==0 && *i>240u8 {
            f = true;
        }
        if res.len()!=0 || *i != 0u8 {
            res.push(*i);
        }
    }
    if f {
        res.push(0);
    }
    println!("res:{}", hex::encode(res));
    assert_eq!(false, true);
}

#[test]
fn timestamp2() {

}
