use bs_flow_observation::{
    adapters,
    graph::{project, Node},
    model::*,
    parse_observation,
};
use serde_json::{json, Value};
use std::path::PathBuf;
fn read(name: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures/omnia")
            .join(name),
    )
    .unwrap()
}
fn eval(now: &str) -> Evaluation {
    Evaluation::new(now, 30_000).unwrap()
}
fn initial() -> Evaluation {
    eval("2026-09-06T00:00:10Z")
}
fn fixture(name: &str) -> Vec<Evidence> {
    adapters::fixture(&read(name), &initial())
        .unwrap()
        .observations
}

#[test]
fn complete_evidence_and_typed_authority_survive_projection() {
    let input = fixture("equivalent-win32.json");
    let graph = project(&input, &initial()).unwrap();
    assert_eq!(graph.authority, Authority::ObservationOnly);
    assert_eq!(
        graph.flows[0].relation,
        bs_flow_observation::graph::EdgeRelation::EndpointAssociation
    );
    assert_eq!(
        graph.flows[0].traffic_direction,
        bs_flow_observation::graph::TrafficDirection::Unknown
    );
    assert_eq!(graph.flows[0].observation, *input[0].observation());
    assert_eq!(graph.flows[0].origin, Origin::Fixture);
    assert_eq!(graph.nodes.len(), 3);
}

#[test]
fn display_masking_does_not_change_graph_identity_or_observed_endpoints() {
    let original = project(&fixture("equivalent-win32.json"), &initial()).unwrap();
    for name in ["masked-display.json", "inert-action-text.json"] {
        let changed = project(&fixture(name), &initial()).unwrap();
        assert_eq!(original.nodes, changed.nodes);
        assert_eq!(original.flows[0].id, changed.flows[0].id);
        assert_eq!(
            original.flows[0].observation.destination,
            changed.flows[0].observation.destination
        );
        assert_eq!(
            original.flows[0].observation.route,
            changed.flows[0].observation.route
        );
    }
}

#[test]
fn reprojection_downgrades_age_and_cannot_promote_unknown_or_stale() {
    let input = fixture("equivalent-win32.json");
    let graph = project(&input, &eval("2026-09-06T00:00:31Z")).unwrap();
    assert_eq!(graph.flows[0].observation.freshness, Freshness::Stale);
    assert_eq!(graph.flows[0].declared_freshness, Freshness::Fresh);
    assert_eq!(
        graph.flows[0].observation.observed_at,
        input[0].observation().observed_at
    );
    for (name, expected) in [
        ("declared-stale.json", Freshness::Stale),
        ("future-telemetry.json", Freshness::Unknown),
        ("declared-unknown.json", Freshness::Unknown),
    ] {
        let graph = project(&fixture(name), &eval("2026-09-06T00:00:15Z")).unwrap();
        assert_eq!(graph.flows[0].observation.freshness, expected);
    }
}

#[test]
fn unknown_identity_is_absent_and_reused_pid_does_not_share_process_node() {
    let mut input = fixture("equivalent-win32.json");
    let mut new_birth: Value = serde_json::from_str(&read("pid-reused-new-birth.json")).unwrap();
    new_birth["snapshot_id"] = json!("later");
    input.extend(
        adapters::fixture(&new_birth.to_string(), &initial())
            .unwrap()
            .observations,
    );
    let graph = project(&input, &initial()).unwrap();
    assert_ne!(graph.flows[0].process_node, graph.flows[1].process_node);
    for name in ["pid-rebound.json", "missing-process.json"] {
        let graph = project(&fixture(name), &initial()).unwrap();
        assert!(graph.flows[0].process_node.is_none());
        assert!(graph
            .nodes
            .iter()
            .all(|n| matches!(n, Node::Endpoint { .. })));
    }
}

#[test]
fn conflicting_identity_rejects_duplicates_are_idempotent_and_order_is_deterministic() {
    let mut input = fixture("equivalent-win32.json");
    let one = project(&input, &initial()).unwrap();
    input.extend(input.clone());
    assert_eq!(one, project(&input, &initial()).unwrap());
    input.extend(fixture("pid-reused-new-birth.json"));
    assert!(project(&input, &initial()).is_err());
    let mut input = fixture("equivalent-win32.json");
    input.extend(fixture("tuple-reuse.json"));
    let forward = project(&input, &initial()).unwrap();
    input.reverse();
    assert_eq!(forward, project(&input, &initial()).unwrap());
}

#[test]
fn imported_pid_only_claims_never_merge_process_nodes() {
    let mut value: Value = serde_json::from_str(&read("wire-valid.json")).unwrap();
    value["process_ref"] = json!("pid:42");
    let first = parse_observation(&value.to_string(), &initial()).unwrap();
    value["flow_id"] = json!("other-flow");
    let second = parse_observation(&value.to_string(), &initial()).unwrap();
    let graph = project(&[first, second], &initial()).unwrap();
    assert_ne!(graph.flows[0].process_node, graph.flows[1].process_node);
    assert!(graph.flows.iter().all(|f| f.origin == Origin::Imported));
}

#[test]
fn platform_graphs_have_equivalent_evidence_and_topology_without_authority_escalation() {
    for platform in ["darwin", "win32", "linux", "openbsd"] {
        let graph = project(&fixture(&format!("equivalent-{platform}.json")), &initial()).unwrap();
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.flows.len(), 1);
        assert_eq!(
            graph.flows[0].observation.source.address.as_deref(),
            Some("192.0.2.10")
        );
        assert_eq!(
            graph.flows[0].observation.destination.address.as_deref(),
            Some("198.51.100.20")
        );
        assert_eq!(graph.flows[0].observation.counters.bytes_up, Some(0));
        assert_eq!(
            graph.flows[0].observation.authority,
            Authority::ObservationOnly
        );
    }
}
