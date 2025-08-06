use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::storage::PasswordStorage;

static STORE: Lazy<Mutex<Option<PasswordStorage>>> = Lazy::new(|| Mutex::new(None));

#[no_mangle]
pub extern "C" fn init_store(path: *const c_char, key_ptr: *const u8) {
    if path.is_null() || key_ptr.is_null() {
        return;
    }

    let c_path = unsafe { CStr::from_ptr(path).to_str().unwrap_or("db") };
    let key = unsafe { std::slice::from_raw_parts(key_ptr, 32) };
    let mut fixed_key = [0u8; 32];
    fixed_key.copy_from_slice(key);

    let store = PasswordStorage::new(c_path, fixed_key);
    *STORE.lock().unwrap() = Some(store);
}

#[no_mangle]
pub extern "C" fn save_password(tag: *const c_char, password: *const c_char) {
    let store = STORE.lock().unwrap();
    if let Some(store) = store.as_ref() {
        if tag.is_null() || password.is_null() {
            return;
        }

        let tag_str = unsafe { CStr::from_ptr(tag).to_str().unwrap_or("") };
        let pass_str = unsafe { CStr::from_ptr(password).to_str().unwrap_or("") };
        store.save_password(tag_str, pass_str);
    }
}

#[no_mangle]
pub extern "C" fn get_password(tag: *const c_char) -> *mut c_char {
    let store = STORE.lock().unwrap();
    if let Some(store) = store.as_ref() {
        if tag.is_null() {
            return std::ptr::null_mut();
        }

        let tag_str = unsafe { CStr::from_ptr(tag).to_str().unwrap_or("") };
        if let Some(pass) = store.get_password(tag_str) {
            return CString::new(pass).unwrap().into_raw();
        }
    }

    std::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn tag_exists(tag: *const c_char) -> bool {
    let store = STORE.lock().unwrap();
    if let Some(store) = store.as_ref() {
        if tag.is_null() {
            return false;
        }

        let tag_str = unsafe { CStr::from_ptr(tag).to_str().unwrap_or("") };
        return store.tag_exists(tag_str);
    }
    false
}

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
