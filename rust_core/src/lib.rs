use rand::{distributions::Alphanumeric, Rng, RngCore};
use std::ffi::{CString, CStr};
use libc::c_char;
use base64::{engine::general_purpose, Engine as _};
use chacha20::cipher::{StreamCipher, NewCipher};
use chacha20::ChaCha20;

#[no_mangle]
pub extern "C" fn generate_password(len: u32) -> *mut c_char {
    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(char::from)
        .collect();

    CString::new(password).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn encrypt_password(ptr: *const c_char) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let plaintext = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let mut key = [0u8; 32];
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut key);
    rand::thread_rng().fill_bytes(&mut nonce);

    let mut buffer = plaintext.as_bytes().to_vec();
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    cipher.apply_keystream(&mut buffer);

    let mut result = Vec::new();
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&buffer);
    result.extend_from_slice(&key);

    let encoded = general_purpose::STANDARD.encode(result);
    CString::new(encoded).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn decrypt_password(ptr: *const c_char) -> *mut c_char {
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(ptr) };
    let encoded = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let data = match general_purpose::STANDARD.decode(encoded) {
        Ok(d) => d,
        Err(_) => return std::ptr::null_mut(),
    };

    if data.len() < 12 + 32 {
        return std::ptr::null_mut();
    }

    let nonce = &data[..12];
    let ciphertext = &data[12..data.len() - 32];
    let key = &data[data.len() - 32..];

    let mut buffer = ciphertext.to_vec();
    let mut cipher = ChaCha20::new(key.into(), nonce.into());
    cipher.apply_keystream(&mut buffer);

    let decrypted = match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    CString::new(decrypted).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { CString::from_raw(ptr); }
}
