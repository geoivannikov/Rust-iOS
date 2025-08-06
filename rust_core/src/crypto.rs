use base64::{engine::general_purpose, Engine as _};
use chacha20::cipher::{NewCipher, StreamCipher};
use chacha20::ChaCha20;
use rand::{thread_rng, RngCore};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

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
    thread_rng().fill_bytes(&mut key);
    thread_rng().fill_bytes(&mut nonce);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let original = "SuperSecret123!";
        let input = CString::new(original).unwrap();
        let encrypted_ptr = encrypt_password(input.as_ptr());

        assert!(!encrypted_ptr.is_null());

        let decrypted_ptr = decrypt_password(encrypted_ptr);
        assert!(!decrypted_ptr.is_null());

        let decrypted = unsafe { CStr::from_ptr(decrypted_ptr).to_str().unwrap() };
        assert_eq!(decrypted, original);

        unsafe {
            CString::from_raw(encrypted_ptr);
            CString::from_raw(decrypted_ptr as *mut _);
        }
    }

    #[test]
    fn test_encrypt_null_input() {
        let result = encrypt_password(std::ptr::null());
        assert!(result.is_null());
    }

    #[test]
    fn test_decrypt_null_input() {
        let result = decrypt_password(std::ptr::null());
        assert!(result.is_null());
    }

    #[test]
    fn test_decrypt_invalid_base64() {
        let invalid_input = CString::new("!!!not_base64!!!").unwrap();
        let result = decrypt_password(invalid_input.as_ptr());
        assert!(result.is_null());
    }

    #[test]
    fn test_decrypt_short_data() {
        let encoded = base64::engine::general_purpose::STANDARD.encode("short");
        let input = CString::new(encoded).unwrap();
        let result = decrypt_password(input.as_ptr());
        assert!(result.is_null());
    }
}

