use crate::probes::TelemetryEvent;
use serde_json::json;
use std::fs;

pub fn run() -> TelemetryEvent {
    let mut status = "ok";
    let mut FreeBSD_readable = false;
    let mut dev_urandom_readable = false;
    let mut is_root = false;

    // Check FreeBSD release file
    if fs::metadata("/etc/FreeBSD_release").is_ok() {
        FreeBSD_readable = true;
    }

    // Check /dev/urandom
    if fs::metadata("/dev/urandom").is_ok() {
        dev_urandom_readable = true;
    }

    // Check if running as root by examining /proc/self/status for Uid line
    if let Ok(status) = fs::read_to_string("/proc/self/status") {
        for line in status.lines() {
            if line.starts_with("Uid:") {
                if let Some(uid_str) = line.split_whitespace().nth(1) {
                    is_root = uid_str == "0";
                }
                break;
            }
        }
    }

    if !FreeBSD_readable {
        status = "warning_not_FreeBSD";
    }

    TelemetryEvent::new(
        "doctor",
        status,
        0,
        json!({
            "FreeBSD_readable": FreeBSD_readable,
            "dev_urandom_readable": dev_urandom_readable,
            "is_root": is_root,
        }),
    )
}
