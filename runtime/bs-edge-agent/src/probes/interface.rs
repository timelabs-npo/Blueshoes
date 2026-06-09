use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceStatus {
    pub interface: String,
    pub state: String,
    pub exists: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceTelemetry {
    pub event_type: String,
    pub timestamp: u64,
    pub status: Vec<InterfaceStatus>,
}

pub fn run(interfaces: &[&str]) -> InterfaceTelemetry {
    let mut results = Vec::new();

    for &iface in interfaces {
        let path = format!("/sys/class/net/{}/operstate", iface);
        let (exists, state) = match fs::read_to_string(&path) {
            Ok(content) => (true, content.trim().to_string()),
            Err(_) => (false, "unknown".to_string()),
        };

        results.push(InterfaceStatus {
            interface: iface.to_string(),
            state,
            exists,
        });
    }

    InterfaceTelemetry {
        event_type: "probe_interface".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        status: results,
    }
}
