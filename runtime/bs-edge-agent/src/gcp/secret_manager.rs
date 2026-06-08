use super::GcpAuth;
use serde::Deserialize;

pub struct SecretManagerClient {
    auth: GcpAuth,
    project_id: String,
}

#[derive(Deserialize)]
struct SecretPayload {
    data: String, // base64 encoded
}

#[derive(Deserialize)]
struct AccessSecretVersionResponse {
    payload: SecretPayload,
}

impl SecretManagerClient {
    pub fn new(auth: GcpAuth, project_id: &str) -> Self {
        Self {
            auth,
            project_id: project_id.to_string(),
        }
    }

    pub fn access_secret(&self, secret_id: &str, version: &str) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        let url = format!(
            "https://secretmanager.googleapis.com/v1/projects/{}/secrets/{}/versions/{}:access",
            self.project_id, secret_id, version
        );

        let resp = ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()?;

        if resp.status() != 200 {
            return Err(format!("Failed to access secret: {}", resp.status()).into());
        }

        let resp_json: AccessSecretVersionResponse = resp.into_json()?;
        
        use base64::{Engine as _, engine::general_purpose};
        let decoded = general_purpose::STANDARD.decode(&resp_json.payload.data)?;
        
        Ok(String::from_utf8(decoded)?)
    }
}
