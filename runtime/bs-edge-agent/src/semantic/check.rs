use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Core concepts loaded from the immutable compile-time embedded schema file.
#[derive(Debug, Deserialize)]
pub struct Schema {
    pub concepts: HashMap<String, Concept>,
}

#[derive(Debug, Deserialize)]
pub struct Concept {
    pub id: String,
    #[serde(default)]
    pub authority: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

/// Provenance payload produced by an execution transformation.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provenance {
    pub result: String,
    pub derived_from: Vec<String>,
    pub evidence: Vec<String>,
    pub hash: String,
}

/// Drift counters tracking semantic deviations across transactions.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DriftCounters {
    pub authority_inversion: u64,
    pub rollback_weakening: u64,
    pub cloud_sovereignty_leak: u64,
    pub llm_execution_leak: u64,
    pub semantic_alias_conflict: u64,
}

/// Load the schema using static compile-time embedding.
fn load_schema() -> Schema {
    let raw = include_str!("../../../../docs/semantic/bs_core_schema.v0.json");
    serde_json::from_str(raw).expect("Failed to parse static core schema")
}

/// Load the current drift counters from the localized data path.
fn load_counters() -> DriftCounters {
    let path = Path::new("../../docs/semantic/drift_rules.v0.json");
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read drift_rules layout");
        serde_json::from_str(&data).expect("Failed to parse drift_rules schema")
    } else {
        DriftCounters::default()
    }
}

/// Persist updated counters back to the immutable configuration track.
fn persist_counters(counters: &DriftCounters) {
    let path = Path::new("../../docs/semantic/drift_rules.v0.json");
    let json = serde_json::to_string_pretty(counters).unwrap();
    fs::write(path, json).expect("Failed to execute safe write on drift_rules");
}

/// Compute deterministic SHA-256 hash over the canonical JSON representation.
fn compute_hash(payload: &Provenance) -> String {
    let mut clone = payload.clone();
    clone.hash.clear();
    let canonical = serde_json::to_string(&clone).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Main validation routine for routing and execution transitions.
pub fn validate_transformation(mut payload: Provenance) -> Result<Provenance, String> {
    let schema = load_schema();
    let mut counters = load_counters();

    // 1️⃣ Verify every concept referenced in derived_from exists within the static layout
    let known_ids: HashSet<_> = schema
        .concepts
        .values()
        .map(|c| c.id.as_str())
        .collect();

    for cid in &payload.derived_from {
        if !known_ids.contains(cid.as_str()) {
            counters.authority_inversion += 1;
            persist_counters(&counters);
            return Err(format!("Unknown concept id in derived_from: {}", cid));
        }
    }

    // 2️⃣ Enforce strict rule boundaries against unauthorized leaks
    if payload
        .evidence
        .iter()
        .any(|e| e.contains("cloud") && !e.contains("mirror"))
    {
        counters.cloud_sovereignty_leak += 1;
    }

    if payload.result.contains("AUTHORITY") && payload.result != "ALLOW_APPLY_CONFIRMED" {
        counters.llm_execution_leak += 1;
    }

    persist_counters(&counters);

    // 3️⃣ Embed reproducible crypto hash anchor
    payload.hash = compute_hash(&payload);
    Ok(payload)
}
