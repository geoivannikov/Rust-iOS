use std::ffi::{CString};

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut std::os::raw::c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}