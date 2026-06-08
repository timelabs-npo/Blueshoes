use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct RejectionReport {
    pub status: String,
    pub violations: Vec<Violation>,
}

#[derive(Debug, Serialize)]
pub struct Violation {
    pub rule: String,
    pub path: String,
    pub severity: String,
}

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
    let _ = fs::write(path, json);
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
pub fn validate_transformation(mut payload: Provenance) -> Result<Provenance, (i32, RejectionReport)> {
    let schema = load_schema();
    let mut counters = load_counters();
    let mut violations = Vec::new();
    let mut exit_code = 0;

    // 1. Unknown Capability (Code 10)
    let known_ids: HashSet<_> = schema.concepts.values().map(|c| c.id.as_str()).collect();
    for cid in &payload.derived_from {
        if !known_ids.contains(cid.as_str()) {
            counters.semantic_alias_conflict += 1;
            violations.push(Violation {
                rule: format!("unknown_capability: {}", cid),
                path: "$.derived_from".to_string(),
                severity: "critical".to_string(),
            });
            if exit_code == 0 { exit_code = 10; }
        }
    }

    let payload_str = serde_json::to_string(&payload).unwrap().to_lowercase();

    // 2. Cloud overrides 0log (Code 11)
    if payload_str.contains("cloud") && payload_str.contains("override") && payload_str.contains("0log") {
        counters.authority_inversion += 1;
        violations.push(Violation {
            rule: "cloud_may_not_override_0log".to_string(),
            path: "$.authority.source".to_string(),
            severity: "critical".to_string(),
        });
        if exit_code == 0 { exit_code = 11; }
    }

    // 3. Spanner marked as source of truth (Code 11)
    if payload_str.contains("spanner") && payload_str.contains("truth") {
        counters.cloud_sovereignty_leak += 1;
        violations.push(Violation {
            rule: "spanner_as_runtime_truth".to_string(),
            path: "$.evidence".to_string(),
            severity: "critical".to_string(),
        });
        if exit_code == 0 { exit_code = 11; }
    }

    // 4. Missing rollback anchor (Code 12)
    if payload.result.contains("ALLOW_") && !payload.evidence.iter().any(|e| e.contains("rollback_anchor")) {
        counters.rollback_weakening += 1;
        violations.push(Violation {
            rule: "missing_rollback_anchor".to_string(),
            path: "$.evidence".to_string(),
            severity: "critical".to_string(),
        });
        if exit_code == 0 { exit_code = 12; }
    }

    // 5. Evidence hash absent / missing evidence entirely (Code 12)
    if payload.evidence.is_empty() {
        violations.push(Violation {
            rule: "missing_evidence".to_string(),
            path: "$.evidence".to_string(),
            severity: "critical".to_string(),
        });
        if exit_code == 0 { exit_code = 12; }
    }

    // 6. LLM as executor (Code 13)
    if payload_str.contains("llm") && payload_str.contains("executor") {
        counters.llm_execution_leak += 1;
        violations.push(Violation {
            rule: "llm_as_executor".to_string(),
            path: "$.result".to_string(),
            severity: "critical".to_string(),
        });
        if exit_code == 0 { exit_code = 13; }
    }

    if !violations.is_empty() {
        persist_counters(&counters);
        return Err((exit_code, RejectionReport {
            status: "REJECTED".to_string(),
            violations,
        }));
    }

    persist_counters(&counters);
    payload.hash = compute_hash(&payload);
    Ok(payload)
}
