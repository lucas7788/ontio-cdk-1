#![no_std]
#![feature(proc_macro_hygiene)]
extern crate ontio_std as ostd;
use ostd::abi::{Sink, Source};
use ostd::contract::governance;
use ostd::contract::governance::{PeerPoolItem, PeerPoolMap};
use ostd::prelude::*;
use ostd::runtime;

fn get_peer_pool() -> PeerPoolMap {
    governance::get_peer_pool()
}

fn get_peer_info(pk: &[u8]) -> PeerPoolItem {
    governance::get_peer_info(pk)
}

fn get_peer_pool_by_address(addr: &Address) -> PeerPoolMap {
    governance::get_peer_pool_by_address(addr)
}

#[no_mangle]
pub fn invoke() {
    let input = runtime::input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "get_peer_pool" => sink.write(get_peer_pool()),
        "get_peer_info" => {
            let pk = source.read().unwrap();
            sink.write(get_peer_info(pk));
        }
        "get_peer_pool_by_address" => {
            let address = source.read().unwrap();
            sink.write(get_peer_pool_by_address(address));
        }
        _ => panic!("unsupported action!"),
    }

    runtime::ret(sink.bytes())
}

extern crate hexutil;
use hexutil::read_hex;

#[cfg(test)]
mod tests {
    use super::Address;
    use super::*;
    use super::{PeerPoolItem, PeerPoolMap};
    use super::{Sink, Source};

    #[test]
    fn it_works() {
        let ppi = PeerPoolItem {
            index: 1,
            peer_pubkey: vec![0u8; 32],
            address: Address::repeat_byte(1),
            status: 1u8,
            init_pos: 10000u64,
            total_pos: 10000u64,
        };
        let mut sink = Sink::new(64);
        sink.write(&ppi);
        let mut source = Source::new(sink.bytes());
        let ppi2: PeerPoolItem = source.read().unwrap();
        assert_eq!(ppi.total_pos, ppi2.total_pos);

        let bs = read_hex("01000000000000000000000000000000000000000000000000000000000000000140420f0000000000400d030000000000").unwrap_or_default();
        let mut source = Source::new(bs.as_slice());
        let ppi3: PeerPoolItem = source.read().unwrap();
        assert_eq!(ppi3.total_pos, 200000);

        let bs = read_hex("0100000001000000000000000000000000000000000000000000000000000000000000000140420f0000000000400d030000000000").unwrap_or_default();
        let mut source = Source::new(bs.as_slice());
        let ppm: PeerPoolMap = source.read().unwrap();
        assert_eq!(ppm.peer_pool_map.len(), 1);
        assert_eq!(ppm.peer_pool_map[0].total_pos, 200000);
    }
}
