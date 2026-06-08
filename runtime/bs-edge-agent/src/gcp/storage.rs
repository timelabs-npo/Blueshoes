use super::GcpAuth;
use std::fs::File;
use std::io::Read;

pub struct StorageClient {
    auth: GcpAuth,
    bucket: String,
}

impl StorageClient {
    pub fn new(auth: GcpAuth, bucket: &str) -> Self {
        Self {
            auth,
            bucket: bucket.to_string(),
        }
    }

    pub fn upload_file(&self, local_path: &str, destination_object_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let token = self.auth.get_token()?;
        
        let mut file = File::open(local_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let url = format!(
            "https://storage.googleapis.com/upload/storage/v1/b/{}/o?uploadType=media&name={}",
            self.bucket,
            urlencoding::encode(destination_object_name)
        );

        let resp = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", token))
            .set("Content-Type", "application/octet-stream")
            .send_bytes(&contents)?;

        if resp.status() != 200 {
            return Err(format!("Failed to upload to Cloud Storage: {}", resp.status()).into());
        }

        Ok(())
    }
}
