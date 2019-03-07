use crate::types::Address;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

/// Mock of contract execution runtime
#[derive(Default)]
pub struct Runtime{
    pub(crate) inner: Rc<RefCell<RuntimeInner>>,
}

#[derive(Default)]
pub(crate) struct RuntimeInner{
    pub(crate) storage: HashMap<Vec<u8>, Vec<u8>>,
    pub(crate) timestamp: u64,
    pub(crate) block_height: u64,
    pub(crate) caller: Address,
    pub(crate) self_addr:Address,
    pub(crate) witness: Vec<Address>,
    pub(crate) notify: Vec<Vec<u8>>,
    pub(crate) call_output_length:u32,
    pub(crate) debug: Vec<String>,
}

impl Runtime {
    fn call_contract(&self, addr: &Address, input: &[u8]) -> Option<Vec<u8>> {
        None
    }
    fn call_output_length(&self) -> u32 {
        self.inner.borrow().call_output_length
    }
    fn get_call_output(&self, dst: *mut u8) {

    }
    fn storage_write(&self, key: &[u8], val: &[u8]) {
        self.inner.borrow_mut().storage.insert(key.into(), val.to_vec());
    }

    fn storage_read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.borrow().storage.get(key).map(|val| val.to_vec())
    }

    fn storage_delete(&self, key: &[u8]) {
        self.inner.borrow_mut().storage.remove(key);
    }

    fn timestamp(&self) -> u64 {
        self.inner.borrow().timestamp
    }

    fn block_height(&self) -> u64 {
        self.inner.borrow().block_height
    }

    fn address(&self) -> Address {
        self.inner.borrow().self_addr.clone()
    }

    fn caller(&self) -> Address {
        self.inner.borrow().caller.clone()
    }

    fn check_witness(&self, addr: &Address) -> bool {
        self.inner.borrow().witness.iter().position(|wit| wit == addr).is_some()
    }

    fn notify(&self, msg: &[u8]) {
        self.inner.borrow_mut().notify.push(msg.to_vec());
    }

    fn debug(&self, msg: &str) {
        self.inner.borrow_mut().debug.push(msg.to_string())
    }
}

thread_local!(static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default()));

pub fn setup_runtime(runtime: Runtime) {
    RUNTIME.with(|r| *r.borrow_mut() = runtime);
}

mod env {
    use super::*;
    use std::slice;
    use std::ptr;
    use std::u32;
    use std::cmp;

    #[no_mangle]
    pub unsafe extern "C" fn timestamp() -> u64 {
        RUNTIME.with(|r| r.borrow().timestamp())
    }

    #[no_mangle]
    pub unsafe extern "C" fn block_height() -> u64 {
        RUNTIME.with(|r| r.borrow().block_height())
    }

    #[no_mangle]
    pub unsafe extern "C" fn self_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let addr = r.borrow().address();
             ptr::copy(addr.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn caller_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let caller = r.borrow().caller();
            ptr::copy(caller.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn check_witness(addr: *const u8) -> bool {
        let address = Address::from_slice(slice::from_raw_parts(addr, 20));
        RUNTIME.with(|r| r.borrow().check_witness(&address))
    }

    #[no_mangle]
    pub unsafe fn storage_read(key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32) -> u32 {
        let offset = offset as usize;
        let key = slice::from_raw_parts(key, klen as usize);
        let v = RUNTIME.with(|r| r.borrow().storage_read(key));
        match v {
            None => u32::MAX,
            Some(v) => {
                ptr::copy(v.as_slice()[offset..].as_ptr(), val, cmp::min(vlen as usize, v.len() - offset));
                v.len() as u32
            }
        }
    }

    #[no_mangle]
    pub unsafe fn storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32) {
        let key = slice::from_raw_parts(key, klen as usize);
        let val = slice::from_raw_parts(val, vlen as usize);
        RUNTIME.with(|r| r.borrow().storage_write(key, val));
    }

    #[no_mangle]
    pub unsafe fn storage_delete(key: *const u8, klen: u32) {
        let key = slice::from_raw_parts(key, klen as usize);
        RUNTIME.with(|r| r.borrow().storage_delete(key));
    }

    #[no_mangle]
    pub unsafe fn notify(ptr: *const u8, len: u32) {
        let msg = slice::from_raw_parts(ptr, len as usize);
        RUNTIME.with(|r| r.borrow().notify(msg));
    }

    #[no_mangle]
    pub unsafe fn debug(msg: &str) {
        RUNTIME.with(|r| r.borrow().debug(msg));
    }
    #[no_mangle]
    pub unsafe extern "C" fn get_call_output(dest: *mut u8){
        RUNTIME.with(|r| r.borrow().get_call_output(dest))
    }
    #[no_mangle]
    pub unsafe extern "C" fn call_output_length() -> u32 {
        RUNTIME.with(|r| r.borrow().call_output_length())
    }
    #[no_mangle]
    pub unsafe extern "C" fn call_contract(addr: *const u8, input_ptr: *const u8, input_len: u32) -> u32 {
        let address = Address::from_slice(slice::from_raw_parts(addr, 20));
        let input = slice::from_raw_parts(input_ptr, input_len as usize);
        let v= RUNTIME.with(|r| r.borrow().call_contract(&address, input));
        match v {
            None => u32::MAX,
            Some(v) => {
                v.len() as u32
            }
        }
    }
}
