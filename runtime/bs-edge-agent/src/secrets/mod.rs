pub mod local_cache;

use crate::gcp::secret_manager::SecretManagerClient;
use local_cache::SovereignSecretManager;

pub struct LocalFirstSecretProvider {
    cache: SovereignSecretManager,
}

impl LocalFirstSecretProvider {
    pub fn new(cache_path: &str, master_key_bytes: &[u8; 32]) -> Self {
        Self {
            cache: SovereignSecretManager::new(cache_path, master_key_bytes),
        }
    }

    pub fn resolve_secret(
        &mut self,
        secret_id: &str,
        gcp_client: Option<&SecretManagerClient>,
    ) -> Result<String, String> {
        if let Some(cached_secret) = self.cache.get_cached(secret_id) {
            return Ok(cached_secret);
        }

        match gcp_client {
            Some(client) => {
                let upstream_secret = client.fetch_raw_secret(secret_id)?;
                self.cache.insert_and_flush(secret_id, upstream_secret.clone())?;
                Ok(upstream_secret)
            }
            None => Err(format!(
                "Offline Mode Error: Secret {} missing from local vault.",
                secret_id
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_offline_cache_read_works_without_network() {
        let path = "/tmp/test_sovereign_cache.json";
        let _ = fs::remove_file(path);
        let key = [42u8; 32];
        
        let mut provider = LocalFirstSecretProvider::new(path, &key);
        provider.cache.insert_and_flush("my_api_key", "super_secret_value".to_string()).unwrap();

        let mut offline_provider = LocalFirstSecretProvider::new(path, &key);
        let result = offline_provider.resolve_secret("my_api_key", None);
        assert_eq!(result.unwrap(), "super_secret_value");
    }

    #[test]
    fn test_missing_local_cache_returns_explicit_error() {
        let path = "/tmp/test_missing_cache.json";
        let _ = fs::remove_file(path);
        let key = [42u8; 32];

        let mut provider = LocalFirstSecretProvider::new(path, &key);
        let result = provider.resolve_secret("missing_key", None);
        
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Offline Mode Error: Secret missing_key missing from local vault."
        );
    }
}
