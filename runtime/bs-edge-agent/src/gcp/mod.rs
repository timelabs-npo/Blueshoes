pub mod cloud_run;
pub mod firestore;
pub mod logging;
pub mod pubsub;
pub mod secret_manager;
pub mod spanner;
pub mod storage;

use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct ServiceAccountKey {
    pub client_email: String,
    pub private_key: String,
    pub private_key_id: String,
    pub token_uri: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Clone)]
pub struct GcpAuth {
    key: Arc<ServiceAccountKey>,
    cached_token: Arc<Mutex<Option<(String, u64)>>>, // Token, Expiry timestamp
}

impl GcpAuth {
    pub fn new(key_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let key_content = fs::read_to_string(key_path)?;
        let key: ServiceAccountKey = serde_json::from_str(&key_content)?;
        Ok(Self {
            key: Arc::new(key),
            cached_token: Arc::new(Mutex::new(None)),
        })
    }

    pub fn get_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let now = Utc::now().timestamp() as u64;
        
        let mut cache = self.cached_token.lock().unwrap();
        if let Some((token, expires_at)) = &*cache {
            if now < *expires_at - 60 {
                return Ok(token.clone());
            }
        }

        // Generate JWT
        let iat = now as usize;
        let exp = iat + 3600;
        
        let claims = Claims {
            iss: self.key.client_email.clone(),
            scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
            aud: self.key.token_uri.clone(),
            exp,
            iat,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.key.private_key_id.clone());

        let encoding_key = EncodingKey::from_rsa_pem(self.key.private_key.as_bytes())?;
        let jwt = encode(&header, &claims, &encoding_key)?;

        // Exchange for access token
        let response = ureq::post(&self.key.token_uri)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])?;

        let token_resp: TokenResponse = response.into_json()?;
        
        let new_expires_at = now + token_resp.expires_in;
        *cache = Some((token_resp.access_token.clone(), new_expires_at));

        Ok(token_resp.access_token)
    }
}
