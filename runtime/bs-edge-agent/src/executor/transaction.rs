use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json::json;

const LOCK_FILE: &str = "/tmp/blueshoes.lock";
const ROLLBACK_DIR: &str = "/tmp/blueshoes/rollbacks"; // /tmp for QEMU compatibility/test env

pub struct TransactionLock {
    _file: File,
}

pub fn acquire_exclusive_lock() -> io::Result<TransactionLock> {
    if !Path::new("/tmp").exists() {
        fs::create_dir_all("/tmp")?;
    }
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(LOCK_FILE)?;
        
    // Attempt to acquire an exclusive lock. If another process has it, this blocks or fails
    match rustix::fs::flock(&file, rustix::fs::FlockOperation::LockExclusive) {
        Ok(_) => Ok(TransactionLock { _file: file }),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "Another bs-edge-agent transaction is already in progress.",
            ));
        }
    }
}

impl Drop for TransactionLock {
    fn drop(&mut self) {
        let _ = rustix::fs::flock(&self._file, rustix::fs::FlockOperation::Unlock);
    }
}

pub fn shift_and_create_snapshot() -> io::Result<()> {
    if !Path::new(ROLLBACK_DIR).exists() {
        fs::create_dir_all(ROLLBACK_DIR)?;
    }

    // Shift existing rollbacks 8 -> 9, 7 -> 8, ..., 0 -> 1
    for i in (0..9).rev() {
        let src = format!("{}/rb_{}.tar.gz", ROLLBACK_DIR, i);
        let dst = format!("{}/rb_{}.tar.gz", ROLLBACK_DIR, i + 1);
        if Path::new(&src).exists() {
            let _ = fs::rename(&src, &dst);
        }
    }

    // Create rb_0.tar.gz covering /etc/config
    let tarball = File::create(format!("{}/rb_0.tar.gz", ROLLBACK_DIR))?;
    let enc = GzEncoder::new(tarball, Compression::default());
    let mut builder = Builder::new(enc);

    // If /etc/config doesn't exist (like on macOS dev machine), we skip or mock
    if Path::new("/etc/config").exists() {
        builder.append_dir_all("config", "/etc/config")?;
    } else {
        // Mock for tests
        fs::create_dir_all("/tmp/blueshoes_mock_config")?;
        File::create("/tmp/blueshoes_mock_config/network")?.write_all(b"mock")?;
        builder.append_dir_all("config", "/tmp/blueshoes_mock_config")?;
    }

    builder.finish()?;
    Ok(())
}

pub fn arm_watchdog(tx_id: &str, timeout_secs: u64) -> io::Result<std::process::Child> {
    let watchdog_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .join("bs-watchdog");
        
    let child = std::process::Command::new(watchdog_path)
        .arg("--tx-id")
        .arg(tx_id)
        .arg("--timeout")
        .arg(timeout_secs.to_string())
        .spawn()?;
        
    Ok(child)
}

pub fn confirm_transaction(tx_id: &str) -> io::Result<()> {
    // A simple IPC mechanism: write a file that the watchdog checks
    let confirm_file = format!("/tmp/blueshoes_confirm_{}", tx_id);
    let mut file = File::create(&confirm_file)?;
    file.write_all(b"CONFIRMED")?;
    Ok(())
}

pub fn generate_tribunal_request_stub(tx_id: &str, intent: &str, target_repo: &str) -> io::Result<()> {
    // In actual implementation, find repo root. For prototype, use local .tasks.
    let tasks_dir = Path::new("../../.tasks");
    let resolved_dir = if tasks_dir.exists() {
        tasks_dir
    } else {
        Path::new(".tasks")
    };
    
    if !resolved_dir.exists() {
        fs::create_dir_all(resolved_dir)?;
    }

    let request = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "schema_id": "urn:tribunal:review-request:v1",
        "request_id": tx_id,
        "milestone": "Prototype",
        "review_mode": "local_manual",
        "review_target": {
            "repo": target_repo,
            "branch": "main",
            "commit": "pending",
            "paths": []
        },
        "risk_context": {
            "mutation_risk": "possible",
            "router_access": true,
            "runtime_change": true,
            "governance_change": false,
            "external_api_dependency": false
        },
        "evidence": {
            "summary": intent,
            "diff_summary": "",
            "test_output_summary": "",
            "artifact_hashes": []
        },
        "questions": [
            "Is this change consistent with Rollback is Sacred?",
            "Does this introduce runtime mutation?",
            "Does this introduce governance creep?",
            "Does this preserve default dry-run behavior?"
        ]
    });

    let pending_file = resolved_dir.join(format!("pending_request_{}.json", tx_id));
    let mut file = File::create(&pending_file)?;
    file.write_all(serde_json::to_string_pretty(&request).unwrap().as_bytes())?;
    
    Ok(())
}
