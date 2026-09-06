#![forbid(unsafe_code)]

use crate::model::*;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Endpoint { id: String, endpoint: Endpoint },
    Process { id: String, process_ref: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FlowEdge {
    pub relation: EdgeRelation,
    pub traffic_direction: TrafficDirection,
    pub id: String,
    pub source_node: String,
    pub destination_node: String,
    pub process_node: Option<String>,
    pub origin: Origin,
    pub declared_freshness: Freshness,
    pub prior_freshness: Freshness,
    pub observation: FlowObservation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FlowGraph {
    pub schema_version: &'static str,
    pub authority: Authority,
    pub projected_at: String,
    pub nodes: Vec<Node>,
    pub flows: Vec<FlowEdge>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EdgeRelation {
    #[serde(rename = "endpoint_association")]
    EndpointAssociation,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum TrafficDirection {
    #[serde(rename = "unknown")]
    Unknown,
}

fn key(value: impl Serialize) -> FlowResult<String> {
    // Unambiguous tuple encoding; this is not substrate canonicalization or a receipt hash.
    serde_json::to_string(&value).map_err(|e| e.to_string())
}

/// Pure, stateless projection: no executor, callbacks, network or persistence handle.
/// Reproject against a new clock to downgrade cached data. Never infer actual routes
/// or process ownership from display annotations or an unscoped imported reference.
pub fn project(evidence: &[Evidence], evaluation: &Evaluation) -> FlowResult<FlowGraph> {
    if evidence.len() > MAX_RECORDS {
        return Err("graph observation limit".into());
    }
    let mut observations = BTreeMap::new();
    for item in evidence {
        validate(&item.observation)?;
        let id = key((
            item.origin,
            item.observation.platform,
            &item.observation.provenance,
            &item.observation.flow_id,
        ))?;
        if let Some(previous) = observations.insert(id, item) {
            if previous != item {
                return Err("conflicting observation identity; no implicit overwrite".into());
            }
        }
    }
    let mut nodes = BTreeMap::new();
    let mut flows = Vec::with_capacity(observations.len());
    for (id, item) in observations {
        let mut observation = item.observation.clone();
        let prior_freshness = observation.freshness;
        observation.freshness = evaluation.freshness(&observation.observed_at, prior_freshness)?;
        let source_node = key(("source", &id))?;
        let destination_node = key(("destination", &id))?;
        // Endpoints are scoped to their supporting observation. Equal display labels,
        // unknown peers or addresses from different collectors never imply a shared entity.
        nodes.insert(
            source_node.clone(),
            Node::Endpoint {
                id: source_node.clone(),
                endpoint: observation.source.clone(),
            },
        );
        nodes.insert(
            destination_node.clone(),
            Node::Endpoint {
                id: destination_node.clone(),
                endpoint: observation.destination.clone(),
            },
        );
        let process_node = match &observation.process_ref {
            Some(reference) => {
                // Only our native/fixture normalizers issue collection-scoped birth refs.
                // Imported references are opaque assertions and remain per-observation.
                let process_id = if item.origin == Origin::Imported {
                    key(("process-assertion", &id, reference))?
                } else {
                    key((
                        "process",
                        item.origin,
                        observation.platform,
                        &observation.provenance,
                        reference,
                    ))?
                };
                nodes.insert(
                    process_id.clone(),
                    Node::Process {
                        id: process_id.clone(),
                        process_ref: reference.clone(),
                    },
                );
                Some(process_id)
            }
            None => None,
        };
        flows.push(FlowEdge {
            relation: EdgeRelation::EndpointAssociation,
            traffic_direction: TrafficDirection::Unknown,
            id,
            source_node,
            destination_node,
            process_node,
            origin: item.origin,
            declared_freshness: item.declared_freshness,
            prior_freshness,
            observation,
        });
    }
    Ok(FlowGraph {
        schema_version: "FlowGraphV1",
        authority: Authority::ObservationOnly,
        projected_at: evaluation.now(),
        nodes: nodes.into_values().collect(),
        flows,
    })
}
