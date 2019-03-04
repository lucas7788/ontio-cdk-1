mod env {
    extern "C" {
        pub fn contract_debug(data: *const u8, len: u32);
    }
}

pub fn debug(msg: &str) {
    unsafe {
        env::contract_debug(msg.as_ptr(), msg.len() as u32);
    }
}
