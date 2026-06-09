use super::GcpAuth;
use base64::{engine::general_purpose, Engine as _};
use serde::Deserialize;

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

        let decoded = general_purpose::STANDARD
            .decode(&resp_json.payload.data)
            .map_err(|e| e.to_string())?;

        String::from_utf8(decoded).map_err(|e| e.to_string())
    }
}
