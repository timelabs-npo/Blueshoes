use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RejectionViolation {
    pub rule: String,
    pub path: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RejectionReport {
    pub status: String,
    pub violations: Vec<RejectionViolation>,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provenance {
    pub result: String,
    pub derived_from: Vec<String>,
    pub evidence: Vec<String>,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DriftCounters {
    pub authority_inversion: u64,
    pub rollback_weakening: u64,
    pub cloud_sovereignty_leak: u64,
    pub llm_execution_leak: u64,
    pub semantic_alias_conflict: u64,
}

fn load_schema() -> Schema {
    let raw = include_str!("../../../../docs/semantic/bs_core_schema.v0.json");
    serde_json::from_str(raw).expect("Failed to parse static core schema")
}

fn load_counters() -> DriftCounters {
    let path = Path::new("../../docs/semantic/drift_rules.v0.json");
    if path.exists() {
        let data = fs::read_to_string(path).expect("Failed to read drift_rules layout");
        serde_json::from_str(&data).expect("Failed to parse drift_rules schema")
    } else {
        DriftCounters::default()
    }
}

fn persist_counters(counters: &DriftCounters) {
    let path = Path::new("../../docs/semantic/drift_rules.v0.json");
    let json = serde_json::to_string_pretty(counters).unwrap();
    let _ = fs::write(path, json);
}

fn compute_hash(payload: &Provenance) -> String {
    let mut clone = payload.clone();
    clone.hash.clear();
    let canonical = serde_json::to_string(&clone).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn validate_transformation(mut payload: Provenance) -> Result<Provenance, (RejectionReport, i32)> {
    let schema = load_schema();
    let mut counters = load_counters();
    let mut violations = Vec::new();

    let known_ids: HashSet<_> = schema
        .concepts
        .values()
        .map(|c| c.id.as_str())
        .collect();

    // Code 10: Unknown Capability / Schema Mismatch
    for cid in &payload.derived_from {
        if !known_ids.contains(cid.as_str()) {
            counters.authority_inversion += 1;
            violations.push(RejectionViolation {
                rule: format!("unknown_capability: {}", cid),
                path: "$.derived_from".to_string(),
                severity: "critical".to_string(),
            });
        }
    }
    if !violations.is_empty() {
        persist_counters(&counters);
        return Err((RejectionReport { status: "REJECTED".to_string(), violations }, 10));
    }

    // Code 11: Invariant Sovereignty Breach (Cloud leaks / Spanner as state truth)
    if payload.evidence.iter().any(|e| e.contains("spanner") || (e.contains("cloud") && !e.contains("mirror"))) {
        counters.cloud_sovereignty_leak += 1;
        violations.push(RejectionViolation {
            rule: if payload.evidence.iter().any(|e| e.contains("spanner")) { "spanner_as_runtime_truth".to_string() } else { "cloud_overrides_local_0log".to_string() },
            path: "$.evidence".to_string(),
            severity: "critical".to_string(),
        });
        persist_counters(&counters);
        return Err((RejectionReport { status: "REJECTED".to_string(), violations }, 11));
    }

    // Code 12: Missing Evidence / Weak Rollback Protection
    if payload.evidence.is_empty() || (payload.result.contains("MUTATION") && !payload.evidence.iter().any(|e| e.contains("rollback_anchor"))) {
        counters.rollback_weakening += 1;
        violations.push(RejectionViolation {
            rule: "missing_rollback_anchor_on_mutation".to_string(),
            path: "$.evidence".to_string(),
            severity: "critical".to_string(),
        });
        persist_counters(&counters);
        return Err((RejectionReport { status: "REJECTED".to_string(), violations }, 12));
    }

    // Code 13: Unknown Authority / LLM Marked as Primary Executor
    if payload.result.contains("AUTHORITY") && payload.result != "ALLOW_APPLY_CONFIRMED" {
        counters.llm_execution_leak += 1;
        violations.push(RejectionViolation {
            rule: "llm_as_unauthorized_executor".to_string(),
            path: "$.result".to_string(),
            severity: "critical".to_string(),
        });
        persist_counters(&counters);
        return Err((RejectionReport { status: "REJECTED".to_string(), violations }, 13));
    }

    payload.hash = compute_hash(&payload);
    Ok(payload)
}
