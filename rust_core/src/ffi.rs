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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[test]
    fn test_free_string_valid_ptr() {
        let s = CString::new("test").unwrap();
        let ptr = s.into_raw();
        free_string(ptr);
    }

    #[test]
    fn test_free_string_null_ptr() {
        let null_ptr: *mut c_char = std::ptr::null_mut();
        free_string(null_ptr);
    }
}
