use super::GcpAuth;
use chrono::Utc;
use serde::Serialize;

pub struct LoggingClient {
    auth: GcpAuth,
    log_name: String,
}

#[derive(Serialize)]
struct LogEntry {
    #[serde(rename = "logName")]
    log_name: String,
    #[serde(rename = "textPayload")]
    text_payload: String,
    timestamp: String,
}

#[derive(Serialize)]
struct WriteLogEntriesRequest {
    entries: Vec<LogEntry>,
}

impl LoggingClient {
    pub fn new(auth: GcpAuth, project_id: &str, log_id: &str) -> Self {
        let log_name = format!(
            "projects/{}/logs/{}",
            project_id,
            urlencoding::encode(log_id)
        );
        Self { auth, log_name }
    }

    pub fn write_log(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        let url = "https://logging.googleapis.com/v2/entries:write";

        let entry = LogEntry {
            log_name: self.log_name.clone(),
            text_payload: message.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        };

        let req_body = WriteLogEntriesRequest {
            entries: vec![entry],
        };

        let resp = ureq::post(url)
            .set("Authorization", &format!("Bearer {}", token))
            .send_json(serde_json::to_value(req_body)?)?;

        if resp.status() != 200 {
            return Err(format!("Failed to write to Cloud Logging: {}", resp.status()).into());
        }

        Ok(())
    }
}
