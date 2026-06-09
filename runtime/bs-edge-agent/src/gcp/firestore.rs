use super::GcpAuth;

pub struct FirestoreClient {
    auth: GcpAuth,
    project_id: String,
    database: String,
}

impl FirestoreClient {
    pub fn new(auth: GcpAuth, project_id: &str) -> Self {
        Self {
            auth,
            project_id: project_id.to_string(),
            database: "(default)".to_string(),
        }
    }

    pub fn get_document(
        &self,
        collection: &str,
        document_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        let url = format!(
            "https://firestore.googleapis.com/v1/projects/{}/databases/{}/documents/{}/{}",
            self.project_id, self.database, collection, document_id
        );

        let resp = ureq::get(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .call()?;

        if resp.status() != 200 {
            return Err(format!("Failed to get Firestore document: {}", resp.status()).into());
        }

        Ok(resp.into_string()?)
    }
}
