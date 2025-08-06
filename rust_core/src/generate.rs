use std::os::raw::c_char;
use rand::{distributions::Alphanumeric, Rng};
use std::ffi::{CString};

#[no_mangle]
pub extern "C" fn generate_password(len: u32) -> *mut c_char {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(char::from)
        .collect();

    CString::new(password).unwrap().into_raw()
}
