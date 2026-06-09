use super::GcpAuth;
use serde::Serialize;

pub struct PubSubClient {
    auth: GcpAuth,
    project_id: String,
    topic: String,
}

#[derive(Serialize)]
struct PubSubMessage {
    data: String, // base64 encoded
}

#[derive(Serialize)]
struct PublishRequest {
    messages: Vec<PubSubMessage>,
}

impl PubSubClient {
    pub fn new(auth: GcpAuth, project_id: &str, topic: &str) -> Self {
        Self {
            auth,
            project_id: project_id.to_string(),
            topic: topic.to_string(),
        }
    }

    pub fn publish(&self, payload: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        let url = format!(
            "https://pubsub.googleapis.com/v1/projects/{}/topics/{}:publish",
            self.project_id, self.topic
        );

        use base64::{engine::general_purpose, Engine as _};
        let b64_data = general_purpose::STANDARD.encode(payload.as_bytes());

        let req_body = PublishRequest {
            messages: vec![PubSubMessage { data: b64_data }],
        };

        let resp = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .send_json(serde_json::to_value(req_body)?)?;

        if resp.status() != 200 {
            return Err(format!("Failed to publish to Pub/Sub: {}", resp.status()).into());
        }

        Ok(())
    }
}
