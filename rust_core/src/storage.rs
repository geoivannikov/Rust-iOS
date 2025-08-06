use chacha20::cipher::{StreamCipher, NewCipher};
use chacha20::ChaCha20;
use rand::RngCore;
use sled::Db;
use base64::{engine::general_purpose, Engine as _};

type AeadKey = [u8; 32];

pub struct PasswordStorage {
    db: Db,
    key: AeadKey,
}

impl PasswordStorage {
    pub fn new(path: &str, key: AeadKey) -> Self {
        let db = sled::open(path).expect("Failed to open db");
        Self { db, key }
    }

    pub fn save_password(&self, tag: &str, password: &str) {
        let mut nonce = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce);

        let mut buffer = password.as_bytes().to_vec();
        let mut cipher = ChaCha20::new(&self.key.into(), &nonce.into());
        cipher.apply_keystream(&mut buffer);

        let mut payload = Vec::new();
        payload.extend_from_slice(&nonce);
        payload.extend_from_slice(&buffer);

        let encoded = general_purpose::STANDARD.encode(payload);
        self.db.insert(tag, encoded.as_bytes()).unwrap();
    }

    pub fn get_password(&self, tag: &str) -> Option<String> {
        let encoded = self.db.get(tag).ok()??;
        let data = general_purpose::STANDARD.decode(encoded).ok()?;

        if data.len() < 12 {
            return None;
        }

        let nonce = &data[..12];
        let ciphertext = &data[12..];

        let mut buffer = ciphertext.to_vec();
        let mut cipher = ChaCha20::new(&self.key.into(), nonce.into());
        cipher.apply_keystream(&mut buffer);

        String::from_utf8(buffer).ok()
    }

    pub fn tag_exists(&self, tag: &str) -> bool {
        self.db.contains_key(tag).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn make_store() -> PasswordStorage {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test_db");
        let key = [42u8; 32];
        PasswordStorage::new(path.to_str().unwrap(), key)
    }

    #[test]
    fn test_save_and_get_password() {
        let store = make_store();

        store.save_password("tag1", "mypassword");
        let result = store.get_password("tag1");

        assert_eq!(result.as_deref(), Some("mypassword"));
    }

    #[test]
    fn test_tag_exists() {
        let store = make_store();

        assert!(!store.tag_exists("nonexistent"));
        store.save_password("tag2", "secret");
        assert!(store.tag_exists("tag2"));
    }

    #[test]
    fn test_invalid_tag_returns_none() {
        let store = make_store();

        let result = store.get_password("missing");
        assert!(result.is_none());
    }
}
