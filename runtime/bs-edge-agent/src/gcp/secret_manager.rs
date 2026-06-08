use super::GcpAuth;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use rand::Rng;
use rand::RngExt;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Deserialize)]
struct SecretPayload {
    data: String, // base64 encoded
}

#[derive(Deserialize)]
struct AccessSecretVersionResponse {
    payload: SecretPayload,
}

pub struct SecretManagerClient {
    auth: GcpAuth,
    project_id: String,
}

impl SecretManagerClient {
    pub fn new(auth: GcpAuth, project_id: &str) -> Self {
        Self {
            auth,
            project_id: project_id.to_string(),
        }
    }

    pub fn fetch_raw_secret(&self, secret_id: &str) -> Result<String, String> {
        // We fetch 'latest' version by default for simplicity in this sovereign client
        let token = self.auth.get_token().map_err(|e| e.to_string())?;
        let url = format!(
            "https://secretmanager.googleapis.com/v1/projects/{}/secrets/{}/versions/latest:access",
            self.project_id, secret_id
        );

        let resp = ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()
            .map_err(|e| e.to_string())?;

        if resp.status() != 200 {
            return Err(format!("Failed to access secret: {}", resp.status()));
        }

        let resp_json: AccessSecretVersionResponse = resp.into_json().map_err(|e| e.to_string())?;
        
        let decoded = general_purpose::STANDARD.decode(&resp_json.payload.data).map_err(|e| e.to_string())?;
        
        String::from_utf8(decoded).map_err(|e| e.to_string())
    }
}

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

    /// Attempts to read the hermetic secret store from disk. 
    /// If the cloud is dark, this keeps the workspace fully functional.
    fn hydrate_from_local_disk(&mut self) -> Result<(), String> {
        if !Path::new(&self.local_cache_path).exists() {
            return Ok(());
        }

        let mut file = File::open(&self.local_cache_path).map_err(|e| e.to_string())?;
        let mut encoded = Vec::new();
        file.read_to_end(&mut encoded).map_err(|e| e.to_string())?;

        let vault: EncryptedSecretVault = serde_json::from_slice(&encoded).map_err(|e| e.to_string())?;
        let nonce = Nonce::from_slice(&vault.nonce);
        
        let plaintext_bytes = self.cipher
            .decrypt(nonce, vault.ciphertext.as_slice())
            .map_err(|_| "Cryptographic key error: Unable to unlock local sovereign secrets.".to_string())?;

        self.memory_cache = serde_json::from_reader(plaintext_bytes.as_slice()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Resolution strategy: Prioritizes local memory/cache. Falls back to GCP 
    /// only if the network is available and the key is missing.
    pub fn resolve_secret(&mut self, secret_id: &str, gcp_client: Option<&SecretManagerClient>) -> Result<String, String> {
        if let Some(cached_secret) = self.memory_cache.get(secret_id) {
            return Ok(cached_secret.clone());
        }

        // If local lookup fails, pull from upstream only if explicitly instructed/connected
        match gcp_client {
            Some(client) => {
                let upstream_secret = client.fetch_raw_secret(secret_id)?;
                self.memory_cache.insert(secret_id.to_string(), upstream_secret.clone());
                self.flush_to_disk()?;
                Ok(upstream_secret)
            },
            None => Err(format!("Offline Mode Error: Secret {} missing from local vault.", secret_id))
        }
    }

    fn flush_to_disk(&self) -> Result<(), String> {
        let plaintext_json = serde_json::to_vec(&self.memory_cache).map_err(|e| e.to_string())?;
        let mut rng = rand::rng();
        let mut nonce_bytes = [0u8; 12];
        rng.fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, plaintext_json.as_slice()).map_err(|e| e.to_string())?;
        let vault = EncryptedSecretVault { nonce: nonce_bytes.to_vec(), ciphertext };
        let serialized_vault = serde_json::to_vec(&vault).map_err(|e| e.to_string())?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.local_cache_path)
            .map_err(|e| e.to_string())?;
            
        file.write_all(&serialized_vault).map_err(|e| e.to_string())?;
        Ok(())
    }
}
