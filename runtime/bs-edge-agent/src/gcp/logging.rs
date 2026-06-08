use super::GcpAuth;
use serde::Serialize;
use chrono::Utc;

pub struct LoggingClient {
    auth: GcpAuth,
    project_id: String,
    log_name: String,
}

#[derive(Serialize)]
struct LogEntry {
    logName: String,
    textPayload: String,
    timestamp: String,
}

#[derive(Serialize)]
struct WriteLogEntriesRequest {
    entries: Vec<LogEntry>,
}

impl LoggingClient {
    pub fn new(auth: GcpAuth, project_id: &str, log_id: &str) -> Self {
        let log_name = format!("projects/{}/logs/{}", project_id, urlencoding::encode(log_id));
        Self {
            auth,
            project_id: project_id.to_string(),
            log_name,
        }
    }

    pub fn write_log(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        let url = "https://logging.googleapis.com/v2/entries:write";

        let entry = LogEntry {
            logName: self.log_name.clone(),
            textPayload: message.to_string(),
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
