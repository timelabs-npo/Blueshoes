use bs_flow_observation::{
    adapters::{self, FlowAdapter},
    model::*,
    parse_observation, Evaluation,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/omnia")
}
fn read(name: &str) -> String {
    std::fs::read_to_string(root().join(name)).unwrap()
}
fn eval() -> Evaluation {
    Evaluation::new("2026-09-06T00:00:10Z", 30_000).unwrap()
}
fn native(name: &str) -> adapters::ObservationBatch {
    adapters::fixture(&read(name), &eval()).unwrap()
}

#[test]
fn pinned_corpus_and_all_49_independent_expectations() {
    let lock: Value = serde_json::from_str(
        &std::fs::read_to_string(root().parent().unwrap().join("omnia-lock.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(lock["commit"], "30002c67533258691203391b4f0c30a3125d8e23");
    let files = lock["files"].as_object().unwrap();
    assert_eq!(std::fs::read_dir(root()).unwrap().count(), files.len());
    for (name, hash) in files {
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(std::fs::read(root().join(name)).unwrap())
            ),
            *hash,
            "{name}"
        );
    }
    let manifest: Value = serde_json::from_str(&read("manifest.json")).unwrap();
    let evaluation = Evaluation::new(
        manifest["now"].as_str().unwrap(),
        manifest["max_age_ms"].as_i64().unwrap(),
    )
    .unwrap();
    let cases = manifest["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 49);
    let schema = std::fs::read(root().join("flow-observation.schema.json")).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(schema)),
        manifest["schema_sha256"]
    );
    for case in cases {
        let input = read(case["path"].as_str().unwrap());
        assert_eq!(
            format!("{:x}", Sha256::digest(input.as_bytes())),
            case["sha256"],
            "{}",
            case["id"]
        );
        let result = if case["kind"] == "native" {
            adapters::fixture(&input, &evaluation).map(|b| b.observations)
        } else {
            parse_observation(&input, &evaluation).map(|o| vec![o])
        };
        assert_eq!(
            result.is_ok(),
            case["accepted"].as_bool().unwrap(),
            "{}: {:?}",
            case["id"],
            result
        );
        if let Ok(items) = result {
            let o = items[0].observation();
            let expected = case["expected"].as_object().unwrap();
            for (key, value) in expected {
                let actual = match key.as_str() {
                    "freshness" => json!(o.freshness),
                    "process_bound" => json!(o.process_ref.is_some()),
                    "bytes_up" => json!(o.counters.bytes_up),
                    "bytes_down" => json!(o.counters.bytes_down),
                    "destination_address" => json!(o.destination.address),
                    "display_mode" => json!(o.display_location.as_ref().unwrap().mode),
                    _ => panic!("unhandled independent expectation {key}"),
                };
                assert_eq!(&actual, value, "{}: {key}", case["id"]);
            }
            assert_eq!(o.authority, Authority::ObservationOnly);
        }
    }
}

#[test]
fn equivalent_platforms_preserve_semantics_and_cannot_claim_native_execution() {
    let baseline = native("equivalent-win32.json");
    for p in ["darwin", "win32", "linux", "openbsd"] {
        let batch = native(&format!("equivalent-{p}.json"));
        assert_eq!(batch.native_gate, adapters::Gate::NotExecuted);
        let mut semantic = serde_json::to_value(batch.observations[0].observation()).unwrap();
        let mut expected = serde_json::to_value(baseline.observations[0].observation()).unwrap();
        for doc in [&mut semantic, &mut expected] {
            for key in ["platform", "flow_id", "process_ref", "provenance"] {
                doc.as_object_mut().unwrap().remove(key);
            }
        }
        assert_eq!(semantic, expected);
        assert_eq!(batch.observations[0].origin(), Origin::Fixture);
        assert!(batch.observations[0]
            .observation()
            .process_ref
            .as_ref()
            .unwrap()
            .contains(p));
    }
}

#[test]
fn pid_reuse_and_tuple_reuse_are_distinct_and_rebinding_loses_identity() {
    let old = native("equivalent-win32.json");
    let reused = native("pid-reused-new-birth.json");
    assert_ne!(
        old.observations[0].observation().process_ref,
        reused.observations[0].observation().process_ref
    );
    assert_ne!(
        old.observations[0].observation().flow_id,
        native("tuple-reuse.json").observations[0]
            .observation()
            .flow_id
    );
    assert!(native("pid-rebound.json").observations[0]
        .observation()
        .process_ref
        .is_none());
}

#[test]
fn concrete_adapters_reject_wrong_platform_and_keep_native_stubs_explicit() {
    let darwin = adapters::DarwinAdapter;
    assert!(darwin
        .fixture(&read("equivalent-win32.json"), &eval())
        .is_err());
    for adapter in [
        &darwin as &dyn FlowAdapter,
        &adapters::LinuxAdapter,
        &adapters::OpenBsdAdapter,
    ] {
        let batch = adapter.collect().unwrap();
        assert_eq!(batch.native_gate, adapters::Gate::NotExecuted);
        assert!(batch.observations.is_empty());
        assert!(!batch.gaps.is_empty());
    }
}

#[test]
fn imported_provenance_is_an_assertion_and_cannot_forge_local_origin() {
    let mut value: Value = serde_json::from_str(&read("wire-valid.json")).unwrap();
    value["provenance"]["source_kind"] = json!("native_tcp_table");
    let evidence = parse_observation(&value.to_string(), &eval()).unwrap();
    assert_eq!(evidence.origin(), Origin::Imported);
    value["origin"] = json!("native_local_query");
    assert!(parse_observation(&value.to_string(), &eval()).is_err());
}

#[test]
fn input_limits_and_duplicate_nested_keys_fail_closed() {
    assert!(parse_observation(&" ".repeat(MAX_INPUT_BYTES + 1), &eval()).is_err());
    let duplicate =
        read("wire-valid.json").replace("\"bytes_up\": 0", "\"bytes_up\": 0, \"bytes_up\": 0");
    assert!(parse_observation(&duplicate, &eval()).is_err());
    let mut value: Value = serde_json::from_str(&read("equivalent-win32.json")).unwrap();
    value["records"] = Value::Array(vec![value["records"][0].clone(); MAX_RECORDS + 1]);
    assert!(adapters::fixture(&value.to_string(), &eval()).is_err());
}
