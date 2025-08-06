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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_generate_password_length() {
        let ptr = generate_password(16);
        assert!(!ptr.is_null());

        let c_str = unsafe { CStr::from_ptr(ptr) };
        let password = c_str.to_str().unwrap();
        assert_eq!(password.len(), 16);

        unsafe {
            CString::from_raw(ptr);
        }
    }

    #[test]
    fn test_generate_password_zero_length() {
        let ptr = generate_password(0);
        assert!(!ptr.is_null());

        let c_str = unsafe { CStr::from_ptr(ptr) };
        let password = c_str.to_str().unwrap();
        assert_eq!(password.len(), 0);

        unsafe {
            CString::from_raw(ptr);
        }
    }
}
