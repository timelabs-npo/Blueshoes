use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct MtuStatus {
    pub interface: String,
    pub mtu: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MtuTelemetry {
    pub event_type: String,
    pub timestamp: u64,
    pub status: Vec<MtuStatus>,
}

pub fn run(interfaces: &[&str]) -> MtuTelemetry {
    let mut results = Vec::new();

    for &iface in interfaces {
        let path = format!("/sys/class/net/{}/mtu", iface);
        let mtu = match fs::read_to_string(&path) {
            Ok(content) => content.trim().parse::<u32>().ok(),
            Err(_) => None,
        };

        results.push(MtuStatus {
            interface: iface.to_string(),
            mtu,
        });
    }

    MtuTelemetry {
        event_type: "probe_mtu".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        status: results,
    }
}
