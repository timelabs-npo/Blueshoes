use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedSecretVault {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

pub struct SovereignSecretManager {
    local_cache_path: String,
    cipher: ChaCha20Poly1305,
    memory_cache: HashMap<String, String>,
}

impl SovereignSecretManager {
    pub fn new(cache_path: &str, master_key_bytes: &[u8; 32]) -> Self {
        let key = Key::from_slice(master_key_bytes);
        let cipher = ChaCha20Poly1305::new(key);

        let mut manager = Self {
            local_cache_path: cache_path.to_string(),
            cipher,
            memory_cache: HashMap::new(),
        };

        let _ = manager.hydrate_from_local_disk();
        manager
    }

    fn hydrate_from_local_disk(&mut self) -> Result<(), String> {
        if !Path::new(&self.local_cache_path).exists() {
            return Ok(());
        }

        let mut file = File::open(&self.local_cache_path).map_err(|e| e.to_string())?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).map_err(|e| e.to_string())?;

        let vault: EncryptedSecretVault =
            serde_json::from_slice(&encoded).map_err(|e| e.to_string())?;
        let nonce = Nonce::from_slice(&vault.nonce);

        let plaintext_bytes = self
            .cipher
            .decrypt(nonce, vault.ciphertext.as_slice())
            .map_err(|_| {
                "Cryptographic key error: Unable to unlock local sovereign secrets.".to_string()
            })?;

        self.memory_cache =
            serde_json::from_reader(plaintext_bytes.as_slice()).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_cached(&self, secret_id: &str) -> Option<String> {
        self.memory_cache.get(secret_id).cloned()
    }

    pub fn insert_and_flush(
        &mut self,
        secret_id: &str,
        secret_value: String,
    ) -> Result<(), String> {
        self.memory_cache
            .insert(secret_id.to_string(), secret_value);
        self.flush_to_disk()
    }

    fn flush_to_disk(&self) -> Result<(), String> {
        let plaintext_json = serde_json::to_vec(&self.memory_cache).map_err(|e| e.to_string())?;
        let mut rng = rand::rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext_json.as_slice())
            .map_err(|e| e.to_string())?;
        let vault = EncryptedSecretVault {
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        };
        let serialized_vault = serde_json::to_vec(&vault).map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.local_cache_path)
            .map_err(|e| e.to_string())?;

        file.write_all(&serialized_vault)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
