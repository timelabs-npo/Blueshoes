#![forbid(unsafe_code)]

use crate::model::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
enum NativeVersion {
    NativeFlowSampleV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthKind {
    DarwinStartUs,
    #[serde(rename = "win32_creation_100ns")]
    Win32Creation100ns,
    LinuxStartTicks,
    OpenbsdStartUs,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessEvidence {
    pub pid: u32,
    pub kind: BirthKind,
    #[serde(deserialize_with = "nullable")]
    pub birth_before: Option<u64>,
    #[serde(deserialize_with = "nullable")]
    pub birth_after: Option<u64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeRecord {
    pub record_id: String,
    pub process: Option<ProcessEvidence>,
    pub source: Endpoint,
    pub destination: Endpoint,
    pub protocol: String,
    pub counters: Counters,
    pub policy: Policy,
    pub route: Route,
    pub display_location: Option<DisplayLocation>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSample {
    schema_version: NativeVersion,
    pub platform: Platform,
    pub collector_scope: String,
    pub snapshot_id: String,
    pub observed_at: String,
    pub freshness: Freshness,
    pub records: Vec<NativeRecord>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Gate {
    #[serde(rename = "PASS")]
    Pass,
    #[serde(rename = "NOT_EXECUTED")]
    NotExecuted,
}

#[derive(Clone, Debug, Serialize)]
pub struct ObservationBatch {
    pub platform: Platform,
    pub native_gate: Gate,
    pub gaps: Vec<String>,
    pub observations: Vec<Evidence>,
}

/// Host adapters expose observation only: no executor/capability/callback parameter.
pub trait FlowAdapter {
    fn platform(&self) -> Platform;
    fn collect(&self, evaluation: &Evaluation) -> FlowResult<ObservationBatch>;
    fn fixture(&self, json: &str, evaluation: &Evaluation) -> FlowResult<ObservationBatch> {
        let sample: NativeSample = decode(json)?;
        if sample.platform != self.platform() {
            return Err("adapter/platform mismatch".into());
        }
        let observations = normalize(sample, evaluation, Origin::Fixture)?;
        Ok(ObservationBatch {
            platform: self.platform(),
            native_gate: Gate::NotExecuted,
            gaps: vec!["Fixture execution does not qualify a native collector".into()],
            observations,
        })
    }
}

macro_rules! stub {
    ($name:ident, $platform:ident) => {
        pub struct $name;
        impl FlowAdapter for $name {
            fn platform(&self) -> Platform {
                Platform::$platform
            }
            fn collect(&self, _: &Evaluation) -> FlowResult<ObservationBatch> {
                Ok(not_executed(self.platform()))
            }
        }
    };
}
stub!(DarwinAdapter, Darwin);
stub!(Win32Adapter, Win32);
stub!(LinuxAdapter, Linux);
stub!(OpenBsdAdapter, OpenBsd);

pub fn fixture(json: &str, evaluation: &Evaluation) -> FlowResult<ObservationBatch> {
    let sample: NativeSample = decode(json)?;
    let platform = sample.platform;
    let observations = normalize(sample, evaluation, Origin::Fixture)?;
    Ok(ObservationBatch {
        platform,
        native_gate: Gate::NotExecuted,
        gaps: vec!["Fixture execution does not qualify a native collector".into()],
        observations,
    })
}

fn not_executed(platform: Platform) -> ObservationBatch {
    ObservationBatch {
        platform,
        native_gate: Gate::NotExecuted,
        gaps: vec![format!("{} native collector NOT_EXECUTED", platform.name())],
        observations: vec![],
    }
}

fn token(value: &str) -> FlowResult<()> {
    if value.is_empty()
        || value.len() > 256
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_.-".contains(&b))
    {
        return Err("invalid collector/snapshot/record token".into());
    }
    Ok(())
}

pub(crate) fn identity(
    platform: Platform,
    scope: &str,
    process: &Option<ProcessEvidence>,
) -> FlowResult<Option<String>> {
    let Some(p) = process else { return Ok(None) };
    let expected = match platform {
        Platform::Darwin => BirthKind::DarwinStartUs,
        Platform::Win32 => BirthKind::Win32Creation100ns,
        Platform::Linux => BirthKind::LinuxStartTicks,
        Platform::OpenBsd => BirthKind::OpenbsdStartUs,
        Platform::NetBsd => return Err("NetBSD native input is outside this profile".into()),
    };
    if p.kind != expected {
        return Err("process birth kind/platform mismatch".into());
    }
    Ok(match (p.pid, p.birth_before, p.birth_after) {
        (pid, Some(before), Some(after)) if pid != 0 && before != 0 && before == after => Some(
            serde_json::to_string(&(platform, scope, pid, before)).map_err(|e| e.to_string())?,
        ),
        _ => None,
    })
}

pub(crate) fn normalize(
    sample: NativeSample,
    evaluation: &Evaluation,
    origin: Origin,
) -> FlowResult<Vec<Evidence>> {
    let _ = sample.schema_version;
    if sample.platform == Platform::NetBsd {
        return Err("NetBSD native input is outside this profile".into());
    }
    token(&sample.collector_scope)?;
    token(&sample.snapshot_id)?;
    if sample.records.len() > MAX_RECORDS {
        return Err("too many records".into());
    }
    let freshness = evaluation.freshness(&sample.observed_at, sample.freshness)?;
    let mut ids = HashSet::new();
    sample
        .records
        .into_iter()
        .map(|record| {
            token(&record.record_id)?;
            if !ids.insert(record.record_id.clone()) {
                return Err("duplicate native record ID".into());
            }
            let process_ref = identity(sample.platform, &sample.collector_scope, &record.process)?;
            let observation = FlowObservation {
                schema_version: SchemaVersion::V1,
                flow_id: serde_json::to_string(&(
                    sample.platform,
                    &sample.collector_scope,
                    &sample.snapshot_id,
                    &record.record_id,
                ))
                .map_err(|e| e.to_string())?,
                platform: sample.platform,
                observed_at: sample.observed_at.clone(),
                freshness,
                process_ref,
                source: record.source,
                destination: record.destination,
                protocol: record.protocol.to_ascii_lowercase(),
                counters: record.counters,
                policy: record.policy,
                route: record.route,
                display_location: record.display_location,
                provenance: Provenance {
                    adapter: format!("blueshoes.flow.{}", sample.platform.name()),
                    adapter_version: env!("CARGO_PKG_VERSION").into(),
                    source_kind: if origin == Origin::Native {
                        "native_tcp_table"
                    } else {
                        "fixture"
                    }
                    .into(),
                },
                authority: Authority::ObservationOnly,
            };
            validate(&observation)?;
            Ok(Evidence {
                observation,
                origin,
                declared_freshness: sample.freshness,
            })
        })
        .collect()
}
