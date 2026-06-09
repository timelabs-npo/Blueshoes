use super::GcpAuth;

pub struct CloudRunClient {
    auth: GcpAuth,
    endpoint_url: String,
}

impl CloudRunClient {
    pub fn new(auth: GcpAuth, endpoint_url: &str) -> Self {
        Self {
            auth,
            endpoint_url: endpoint_url.to_string(),
        }
    }

    pub fn analyze_packet(
        &self,
        packet_header: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;

        let resp = ureq::post(&self.endpoint_url)
            .set("Authorization", &format!("Bearer {}", token))
            .set("Content-Type", "application/json")
            .send_string(packet_header)?;

        if resp.status() != 200 {
            return Err(format!("Cloud Run analysis failed: {}", resp.status()).into());
        }

        Ok(resp.into_string()?)
    }
}
