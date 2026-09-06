#![forbid(unsafe_code)]

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize};

pub type FlowResult<T> = Result<T, String>;
pub const MAX_INPUT_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_RECORDS: usize = 10_000;

macro_rules! wire_enum {
    ($name:ident { $($variant:ident => $wire:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub enum $name { $(#[serde(rename = $wire)] $variant),+ }
    };
}
wire_enum!(Platform { Darwin => "darwin", Win32 => "win32", Linux => "linux", OpenBsd => "openbsd", NetBsd => "netbsd" });
wire_enum!(Authority { ObservationOnly => "observation_only" });
wire_enum!(SchemaVersion { V1 => "FlowObservationV1" });
wire_enum!(Freshness { Fresh => "fresh", Stale => "stale", Unknown => "unknown" });
wire_enum!(AddressState { Known => "known", Partial => "partial", Unknown => "unknown" });
wire_enum!(MatchState { Matched => "matched", Unmatched => "unmatched", Unknown => "unknown" });
wire_enum!(RouteState { Observed => "observed", Partial => "partial", Unknown => "unknown" });
wire_enum!(DisplayMode { Real => "real", Derived => "derived", Masked => "masked", Unknown => "unknown" });
wire_enum!(Origin { Imported => "imported_assertion", Fixture => "fixture", Native => "native_local_query" });

impl Platform {
    pub fn name(self) -> &'static str {
        match self {
            Self::Darwin => "darwin",
            Self::Win32 => "win32",
            Self::Linux => "linux",
            Self::OpenBsd => "openbsd",
            Self::NetBsd => "netbsd",
        }
    }
}

/// Forces required-but-nullable properties to reject missing keys.
pub(crate) fn nullable<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(de)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub address_state: AddressState,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub domain: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Counters {
    #[serde(deserialize_with = "nullable")]
    pub bytes_up: Option<u64>,
    #[serde(deserialize_with = "nullable")]
    pub bytes_down: Option<u64>,
    pub rate_up_bps: Option<u64>,
    pub rate_down_bps: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub match_state: MatchState,
    pub rule_ref: Option<String>,
    pub decision: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Route {
    pub state: RouteState,
    pub interface_ref: Option<String>,
    pub egress_ref: Option<String>,
    pub tunnel_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplayLocation {
    pub mode: DisplayMode,
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub adapter: String,
    pub adapter_version: String,
    pub source_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowObservation {
    pub schema_version: SchemaVersion,
    pub flow_id: String,
    pub platform: Platform,
    pub observed_at: String,
    pub freshness: Freshness,
    pub process_ref: Option<String>,
    pub source: Endpoint,
    pub destination: Endpoint,
    pub protocol: String,
    pub counters: Counters,
    pub policy: Policy,
    pub route: Route,
    pub display_location: Option<DisplayLocation>,
    pub provenance: Provenance,
    pub authority: Authority,
}

/// Only parser/adapter entry points construct provenance trust labels.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Evidence {
    pub(crate) observation: FlowObservation,
    pub(crate) origin: Origin,
    pub(crate) declared_freshness: Freshness,
}
impl Evidence {
    pub fn observation(&self) -> &FlowObservation {
        &self.observation
    }
    pub fn origin(&self) -> Origin {
        self.origin
    }
    pub fn declared_freshness(&self) -> Freshness {
        self.declared_freshness
    }
}

#[derive(Clone, Debug)]
pub struct Evaluation {
    now: DateTime<FixedOffset>,
    max_age_ms: i64,
}
impl Evaluation {
    pub fn new(now: &str, max_age_ms: i64) -> FlowResult<Self> {
        if max_age_ms < 0 {
            return Err("negative maximum age".into());
        }
        Ok(Self {
            now: timestamp(now)?,
            max_age_ms,
        })
    }
    pub fn now(&self) -> String {
        self.now.to_rfc3339()
    }
    pub fn freshness(&self, observed_at: &str, declared: Freshness) -> FlowResult<Freshness> {
        let observed = timestamp(observed_at)?;
        let age = self.now.signed_duration_since(observed);
        Ok(if declared == Freshness::Stale {
            Freshness::Stale
        } else if age < chrono::Duration::zero() {
            Freshness::Unknown
        } else if age > chrono::Duration::milliseconds(self.max_age_ms) {
            Freshness::Stale
        } else {
            declared
        })
    }
}

pub(crate) fn timestamp(value: &str) -> FlowResult<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(value).map_err(|_| "invalid RFC3339 timestamp".into())
}

pub(crate) fn decode<T: serde::de::DeserializeOwned>(json: &str) -> FlowResult<T> {
    if json.len() > MAX_INPUT_BYTES {
        return Err("input exceeds 8 MiB".into());
    }
    // Deserialize directly into strict structs, never through a Value that loses duplicate keys.
    serde_json::from_str(json).map_err(|e| format!("invalid observation input: {e}"))
}

pub(crate) fn validate(observation: &FlowObservation) -> FlowResult<()> {
    for value in [
        &observation.flow_id,
        &observation.protocol,
        &observation.provenance.adapter,
        &observation.provenance.adapter_version,
        &observation.provenance.source_kind,
    ] {
        if value.is_empty() {
            return Err("required string is empty".into());
        }
    }
    timestamp(&observation.observed_at)?;
    Ok(())
}

pub fn parse_observation(json: &str, evaluation: &Evaluation) -> FlowResult<Evidence> {
    let mut observation: FlowObservation = decode(json)?;
    validate(&observation)?;
    let declared_freshness = observation.freshness;
    observation.freshness = evaluation.freshness(&observation.observed_at, declared_freshness)?;
    Ok(Evidence {
        observation,
        origin: Origin::Imported,
        declared_freshness,
    })
}
