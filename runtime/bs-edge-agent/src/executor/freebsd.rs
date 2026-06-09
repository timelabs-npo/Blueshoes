#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConceptType {
    #[serde(rename = "http://schema.org/Person")]
    Person,
    #[serde(rename = "http://schema.org/Project")]
    Project,
    #[serde(rename = "http://schema.org/Metric")]
    Metric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub concept: ConceptType,
    #[serde(rename = "http://schema.org")]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceReceipt {
    pub entity_id: String,
    pub timestamp: u64,
    pub operator_signature: String,
    pub transformation_hash: String,
}

pub struct FreeBsdExecutor {
    pub registry: HashMap<String, Entity>,
    pub lineage: Vec<ProvenanceReceipt>,
    pub socket_path: String,
}

impl FreeBsdExecutor {
    pub fn new(socket_path: &str) -> Self {
        Self {
            registry: HashMap::new(),
            lineage: Vec::new(),
            socket_path: socket_path.to_string(),
        }
    }

    /// Verifies the hermetic host environment matches the FreeBSD target
    pub fn verify_environment(&self) -> bool {
        fs::metadata("/etc/freebsd_version").is_ok()
    }

    /// Executes network verification natively via FreeBSD ifconfig
    pub fn get_interface_status(&self, iface: &str) -> Result<String, String> {
        let output = Command::new("ifconfig")
            .arg(iface)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Commits an entity locally with cryptographic provenance and streams via IPC
    pub fn commit_entity(&mut self, entity: Entity, operator: &str) -> ProvenanceReceipt {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Cryptographic generation using standard md5 library primitives
        let mut context = md5::Context::new();
        context.consume(entity.id.as_bytes());
        context.consume(timestamp.to_string().as_bytes());
        context.consume(operator.as_bytes());
        let transformation_hash = format!("{:x}", context.finalize());

        let receipt = ProvenanceReceipt {
            entity_id: entity.id.clone(),
            timestamp,
            operator_signature: operator.to_string(),
            transformation_hash,
        };

        // Stream concept token across IPC channel for real-time drift telemetry
        let concept_str = match entity.concept {
            ConceptType::Person => "Person",
            ConceptType::Project => "Project",
            ConceptType::Metric => "Metric",
        };

        // Hermetic isolation fallback: telemetry server down must never drop execution core
        if let Err(e) = self.send_ipc_telemetry(concept_str) {
            eprintln!("[SOVEREIGN_CORE_WARN] Telemetry pipeline offline: {}", e);
        }

        self.registry.insert(entity.id.clone(), entity);
        self.lineage.push(receipt.clone());

        receipt
    }

    fn send_ipc_telemetry(&self, concept_token: &str) -> std::io::Result<()> {
        let mut stream = UnixStream::connect(&self.socket_path)?;
        writeln!(stream, "{}", concept_token)?;
        stream.flush()?;
        Ok(())
    }

    pub fn export_jsonld(&self) -> Result<String, serde_json::Error> {
        let entities: Vec<&Entity> = self.registry.values().collect();
        serde_json::to_string_pretty(&entities)
    }
}
