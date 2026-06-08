use super::TelemetryEvent;
use serde_json::json;
use std::fs;
use std::time::Instant;

pub fn run() -> TelemetryEvent {
    let start = Instant::now();
    let mut evidence = json!({});

    evidence["os"] = json!(read_os_release());

    let (total, free) = read_meminfo();
    evidence["mem_total_kb"] = json!(total);
    evidence["mem_free_kb"] = json!(free);
    evidence["loadavg"] = json!(read_loadavg());
    evidence["uptime"] = json!(read_uptime());

    let tmp_compliant = crate::mutation::rollback::verify_ephemeral_tmp().is_ok();
    let boot_compliant = crate::mutation::rollback::verify_boot_read_only().is_ok();
    evidence["compliance"] = json!({
        "ephemeral_tmp": tmp_compliant,
        "read_only_boot": boot_compliant,
        "fully_compliant": tmp_compliant && boot_compliant,
    });

    let status = if total.is_some() && tmp_compliant && boot_compliant { "ok" } else { "warn" };

    TelemetryEvent::new(
        "system",
        status,
        start.elapsed().as_millis() as u64,
        evidence,
    )
}

fn read_meminfo() -> (Option<u64>, Option<u64>) {
    let Ok(contents) = fs::read_to_string("/proc/meminfo") else {
        return (None, None);
    };

    let mut total = None;
    let mut free = None;

    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            total = parse_kb(line);
        } else if line.starts_with("MemFree:") {
            free = parse_kb(line);
        }
    }

    (total, free)
}

fn parse_kb(line: &str) -> Option<u64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        parts[1].parse::<u64>().ok()
    } else {
        None
    }
}

fn read_loadavg() -> Option<f64> {
    let Ok(contents) = fs::read_to_string("/proc/loadavg") else {
        return None;
    };
    contents
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
}

fn read_uptime() -> Option<f64> {
    let Ok(contents) = fs::read_to_string("/proc/uptime") else {
        return None;
    };
    contents
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
}

fn read_os_release() -> String {
    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        for line in contents.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    // Fallback to uname if /etc/os-release is missing
    if let Ok(output) = std::process::Command::new("uname").arg("-a").output() {
        if output.status.success() {
            if let Ok(s) = String::from_utf8(output.stdout) {
                return s.trim().to_string();
            }
        }
    }
    "Unknown OS".to_string()
}
