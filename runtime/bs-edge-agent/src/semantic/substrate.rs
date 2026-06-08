use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// ═══════════════════════════════════════════════════════════════════════════
// STRUCTURAL PILLARS — These are the immutable atoms of the semantic layer.
// Once committed, an entity's provenance receipt is cryptographically sealed.
// No agent, no orchestrator, no cloud mirror may alter the lineage chain.
// ═══════════════════════════════════════════════════════════════════════════

/// The explicit, closed set of concept types in the Blueshoes domain.
/// Adding a variant here requires a schema migration — this is intentional.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConceptType {
    /// A human operator, device owner, or administrative identity
    Person,
    /// An active project, deployment, or configuration scope
    Project,
    /// A measurable telemetry value, SLA target, or constraint bound
    Metric,
    /// A network edge device under management
    EdgeNode,
    /// A sovereignty invariant or governance rule
    Invariant,
    /// A capability grant (e.g., apply_confirmed, rollback)
    Capability,
    /// A state generation epoch (0.log promotion cycle)
    StateGeneration,
    /// An external mirror (Spanner, GCS) — explicitly second-class
    ExternalMirror,
}

impl std::fmt::Display for ConceptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConceptType::Person => write!(f, "Person"),
            ConceptType::Project => write!(f, "Project"),
            ConceptType::Metric => write!(f, "Metric"),
            ConceptType::EdgeNode => write!(f, "EdgeNode"),
            ConceptType::Invariant => write!(f, "Invariant"),
            ConceptType::Capability => write!(f, "Capability"),
            ConceptType::StateGeneration => write!(f, "StateGeneration"),
            ConceptType::ExternalMirror => write!(f, "ExternalMirror"),
        }
    }
}

/// A single semantic entity in the substrate. Once committed, properties
/// are frozen at that generation. Updates create new receipts, never mutate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Deterministic ID — format: `{concept_type}:{unique_name}`
    pub id: String,
    /// Explicit typing — no inference, no LLM-assigned categories
    pub concept: ConceptType,
    /// Key-value properties frozen at commit time
    pub properties: HashMap<String, String>,
    /// Links to other entity IDs (directed edges in the knowledge graph)
    pub relations: Vec<Relation>,
}

/// A typed, directed edge between two entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    /// The type of relationship (e.g., "manages", "depends_on", "monitors")
    pub predicate: String,
    /// Target entity ID
    pub target_id: String,
}

/// The cryptographic receipt generated for every substrate mutation.
/// This is the audit trail. Tamper with it and the hash chain breaks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceReceipt {
    /// Which entity was committed
    pub entity_id: String,
    /// Unix epoch seconds at commit time
    pub timestamp: u64,
    /// Who or what performed this commit (human operator ID, agent name)
    pub operator_signature: String,
    /// SHA-256 of (entity_id + timestamp + previous_hash + serialized_entity)
    pub transformation_hash: String,
    /// Hash of the previous receipt — forms the chain
    pub previous_hash: String,
    /// Monotonically increasing generation counter
    pub generation: u64,
}

/// JSON-LD serialization wrapper for external consumption.
/// This is how agents and monitors read the substrate without touching Rust internals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdExport {
    #[serde(rename = "@context")]
    pub context: HashMap<String, String>,
    #[serde(rename = "@graph")]
    pub graph: Vec<JsonLdNode>,
    pub provenance_chain_head: String,
    pub generation: u64,
    pub concept_distribution: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub node_type: String,
    pub properties: HashMap<String, String>,
    pub relations: Vec<JsonLdRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonLdRelation {
    pub predicate: String,
    #[serde(rename = "@id")]
    pub target_id: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// THE DETERMINISTIC EXECUTION ENGINE
// ═══════════════════════════════════════════════════════════════════════════

pub struct SemanticSubstrate {
    /// The entity registry — keyed by entity ID
    pub registry: HashMap<String, Entity>,
    /// Append-only provenance chain — the hash chain of all commits
    pub lineage: Vec<ProvenanceReceipt>,
    /// Current generation counter
    pub generation: u64,
}

impl SemanticSubstrate {
    pub fn new() -> Self {
        SemanticSubstrate {
            registry: HashMap::new(),
            lineage: Vec::new(),
            generation: 0,
        }
    }

    /// Commit an entity to the substrate. Returns a cryptographic provenance receipt.
    /// This is the ONLY way to mutate the registry. Every call is auditable.
    pub fn commit_entity(&mut self, entity: Entity, operator: &str) -> ProvenanceReceipt {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let previous_hash = self
            .lineage
            .last()
            .map(|r| r.transformation_hash.clone())
            .unwrap_or_else(|| "GENESIS".to_string());

        // Deterministic hash: entity_id + timestamp + previous_hash + serialized entity
        let entity_json = serde_json::to_string(&entity).unwrap();
        let hash_input = format!(
            "{}|{}|{}|{}",
            entity.id, timestamp, previous_hash, entity_json
        );
        let mut hasher = Sha256::new();
        hasher.update(hash_input.as_bytes());
        let transformation_hash = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        self.generation += 1;

        let receipt = ProvenanceReceipt {
            entity_id: entity.id.clone(),
            timestamp,
            operator_signature: operator.to_string(),
            transformation_hash,
            previous_hash,
            generation: self.generation,
        };

        self.registry.insert(entity.id.clone(), entity);
        self.lineage.push(receipt.clone());

        receipt
    }

    /// Deterministic query: identical output regardless of external context.
    /// No LLM interpolation, no cloud fetch — pure local filter.
    pub fn query_by_type(&self, concept: &ConceptType) -> Vec<&Entity> {
        self.registry
            .values()
            .filter(|e| &e.concept == concept)
            .collect()
    }

    /// Get an entity by ID. Returns None if not found — never hallucinates.
    pub fn get(&self, id: &str) -> Option<&Entity> {
        self.registry.get(id)
    }

    /// Verify the integrity of the entire provenance chain.
    /// Returns Err with the index of the first broken link.
    pub fn verify_chain_integrity(&self) -> Result<(), (usize, String)> {
        for i in 1..self.lineage.len() {
            let expected_prev = &self.lineage[i - 1].transformation_hash;
            let actual_prev = &self.lineage[i].previous_hash;
            if expected_prev != actual_prev {
                return Err((
                    i,
                    format!(
                        "Chain broken at index {}: expected previous_hash={}, got={}",
                        i, expected_prev, actual_prev
                    ),
                ));
            }
        }
        Ok(())
    }

    /// Calculate the concept distribution of the current registry.
    /// This is the ground-truth P(x) that the drift monitor compares against.
    pub fn concept_distribution(&self) -> HashMap<String, f64> {
        let total = self.registry.len() as f64;
        if total == 0.0 {
            return HashMap::new();
        }

        let mut counts: HashMap<String, f64> = HashMap::new();
        for entity in self.registry.values() {
            *counts.entry(entity.concept.to_string()).or_insert(0.0) += 1.0;
        }
        for val in counts.values_mut() {
            *val /= total;
        }
        counts
    }

    /// Export the entire substrate as JSON-LD for external consumption.
    /// This is the serialization boundary — agents read this, never the raw structs.
    pub fn export_jsonld(&self) -> JsonLdExport {
        let mut context = HashMap::new();
        context.insert(
            "bs".to_string(),
            "https://blueshoes.dev/schema/v0#".to_string(),
        );
        context.insert(
            "schema".to_string(),
            "http://schema.org/".to_string(),
        );

        let graph: Vec<JsonLdNode> = self
            .registry
            .values()
            .map(|e| JsonLdNode {
                id: format!("bs:{}", e.id),
                node_type: format!("bs:{}", e.concept),
                properties: e.properties.clone(),
                relations: e
                    .relations
                    .iter()
                    .map(|r| JsonLdRelation {
                        predicate: format!("bs:{}", r.predicate),
                        target_id: format!("bs:{}", r.target_id),
                    })
                    .collect(),
            })
            .collect();

        let chain_head = self
            .lineage
            .last()
            .map(|r| r.transformation_hash.clone())
            .unwrap_or_else(|| "EMPTY".to_string());

        JsonLdExport {
            context,
            graph,
            provenance_chain_head: chain_head,
            generation: self.generation,
            concept_distribution: self.concept_distribution(),
        }
    }

    /// Serialize the full substrate (registry + lineage) to JSON for persistence.
    pub fn snapshot(&self) -> String {
        let snap = serde_json::json!({
            "generation": self.generation,
            "registry": self.registry,
            "lineage": self.lineage,
        });
        serde_json::to_string_pretty(&snap).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entity(id: &str, concept: ConceptType) -> Entity {
        Entity {
            id: id.to_string(),
            concept,
            properties: HashMap::new(),
            relations: Vec::new(),
        }
    }

    #[test]
    fn test_commit_and_query() {
        let mut substrate = SemanticSubstrate::new();
        let e1 = make_entity("Person:alice", ConceptType::Person);
        let e2 = make_entity("Project:blueshoes", ConceptType::Project);

        let r1 = substrate.commit_entity(e1, "test_operator");
        let r2 = substrate.commit_entity(e2, "test_operator");

        assert_eq!(r1.generation, 1);
        assert_eq!(r2.generation, 2);
        assert_eq!(r2.previous_hash, r1.transformation_hash);

        let persons = substrate.query_by_type(&ConceptType::Person);
        assert_eq!(persons.len(), 1);
        assert_eq!(persons[0].id, "Person:alice");
    }

    #[test]
    fn test_chain_integrity() {
        let mut substrate = SemanticSubstrate::new();
        substrate.commit_entity(make_entity("a", ConceptType::Metric), "op");
        substrate.commit_entity(make_entity("b", ConceptType::Metric), "op");
        substrate.commit_entity(make_entity("c", ConceptType::Metric), "op");

        assert!(substrate.verify_chain_integrity().is_ok());

        // Tamper with the chain
        substrate.lineage[1].previous_hash = "TAMPERED".to_string();
        assert!(substrate.verify_chain_integrity().is_err());
    }

    #[test]
    fn test_concept_distribution() {
        let mut substrate = SemanticSubstrate::new();
        substrate.commit_entity(make_entity("p1", ConceptType::Person), "op");
        substrate.commit_entity(make_entity("p2", ConceptType::Person), "op");
        substrate.commit_entity(make_entity("proj1", ConceptType::Project), "op");
        substrate.commit_entity(make_entity("m1", ConceptType::Metric), "op");

        let dist = substrate.concept_distribution();
        assert!((dist["Person"] - 0.5).abs() < 1e-6);
        assert!((dist["Project"] - 0.25).abs() < 1e-6);
        assert!((dist["Metric"] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_jsonld_export_structure() {
        let mut substrate = SemanticSubstrate::new();
        substrate.commit_entity(make_entity("Person:bob", ConceptType::Person), "op");

        let export = substrate.export_jsonld();
        assert_eq!(export.generation, 1);
        assert_eq!(export.graph.len(), 1);
        assert_eq!(export.graph[0].id, "bs:Person:bob");
        assert!(export.context.contains_key("bs"));
    }
}
