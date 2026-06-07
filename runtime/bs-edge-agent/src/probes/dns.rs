use super::TelemetryEvent;
use serde_json::json;
use std::net::ToSocketAddrs;
use std::time::Instant;

pub fn run(target: &str) -> TelemetryEvent {
    let start = Instant::now();
    let mut evidence = json!({});
    evidence["target"] = json!(target);

    // Using Rust's standard library resolver (blocks, but lightweight and read-only)
    let addr_str = format!("{}:80", target);
    match addr_str.to_socket_addrs() {
        Ok(addrs) => {
            let mut resolved_ips = Vec::new();
            for addr in addrs {
                resolved_ips.push(addr.ip().to_string());
            }
            let is_empty = resolved_ips.is_empty();
            let status = if !is_empty { "ok" } else { "warn" };
            
            evidence["resolved_ips"] = json!(resolved_ips);
            evidence["timing_ms"] = json!(start.elapsed().as_millis() as u64);

            if is_empty {
                evidence["error"] = json!("Resolved to empty IP list");
            }

            TelemetryEvent::new("dns", status, start.elapsed().as_millis() as u64, evidence)
        }
        Err(e) => {
            evidence["error"] = json!(e.to_string());
            TelemetryEvent::new("dns", "fail", start.elapsed().as_millis() as u64, evidence)
        }
    }
}
