use super::TelemetryEvent;
use serde_json::json;
use std::process::Command;
use std::time::Instant;

pub fn run(target: &str) -> TelemetryEvent {
    let start = Instant::now();
    let mut evidence = json!({});
    evidence["target"] = json!(target);

    // Read-only ICMP ping command
    // Strictly FreeBSD/Linux target flags
    let mut cmd = Command::new("ping");
    cmd.args(["-c", "1", "-W", "2", target]);

    match cmd.output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            evidence["stdout"] = json!(stdout);

            let status = if output.status.success() {
                "ok"
            } else {
                "fail"
            };
            if !output.status.success() {
                evidence["stderr"] = json!(String::from_utf8_lossy(&output.stderr).to_string());
            }

            // Extract latency estimate if successful
            for line in stdout.lines() {
                if line.contains("time=") {
                    evidence["latency_line"] = json!(line);
                    break;
                }
            }

            TelemetryEvent::new("icmp", status, start.elapsed().as_millis() as u64, evidence)
        }
        Err(e) => {
            evidence["error"] = json!(e.to_string());
            TelemetryEvent::new("icmp", "fail", start.elapsed().as_millis() as u64, evidence)
        }
    }
}
