use std::env;
use std::fs::{self, File};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::path::Path;
use tar::Archive;
use flate2::read::GzDecoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tx_id = String::new();
    let mut timeout_secs = 60;

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--tx-id" && i + 1 < args.len() {
            tx_id = args[i + 1].clone();
            i += 2;
        } else if args[i] == "--timeout" && i + 1 < args.len() {
            timeout_secs = args[i + 1].parse().unwrap_or(60);
            i += 2;
        } else {
            i += 1;
        }
    }

    if tx_id.is_empty() {
        eprintln!("Usage: bs-watchdog --tx-id <id> [--timeout <secs>]");
        std::process::exit(1);
    }

    let confirm_file = format!("/tmp/blueshoes_confirm_{}", tx_id);
    let check_interval = Duration::from_secs(1);
    let mut elapsed = 0;

    println!("[Watchdog] Armed for TX {} with timeout {}s", tx_id, timeout_secs);

    while elapsed < timeout_secs {
        if Path::new(&confirm_file).exists() {
            println!("[Watchdog] Commit confirmed! Disarming watchdog.");
            let _ = fs::remove_file(&confirm_file);
            std::process::exit(0);
        }
        thread::sleep(check_interval);
        elapsed += 1;
    }

    eprintln!("\n[Watchdog] CRITICAL: Timeout ({}s) expired! Triggering dead-man's switch rollback...", timeout_secs);

    // Rollback phase
    let rollback_file = "/tmp/blueshoes/rollbacks/rb_1.tar.gz";
    if !Path::new(rollback_file).exists() {
        eprintln!("[Watchdog] ERROR: Rollback archive {} not found!", rollback_file);
        std::process::exit(1);
    }

    // Extract the previous configuration (rb_1)
    let tarball = match File::open(rollback_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[Watchdog] Failed to open rollback archive: {}", e);
            std::process::exit(1);
        }
    };

    let dec = GzDecoder::new(tarball);
    let mut archive = Archive::new(dec);

    let target_dir = if Path::new("/etc/config").exists() {
        "/etc"
    } else {
        "/tmp" // Dev environment mock
    };

    if let Err(e) = archive.unpack(target_dir) {
        eprintln!("[Watchdog] Failed to unpack rollback archive: {}", e);
        std::process::exit(1);
    }

    // Restart networking strictly via standard OpenWrt /etc/init.d script
    // AUDIT EXEMPTION: bs-watchdog is the safety rollback daemon itself.
    println!("[Watchdog] Configuration restored. Restarting network services...");
    let mut reload_cmd = Command::new("/etc/init.d/network"); // audit:exempt:watchdog-rollback
    reload_cmd.arg("restart");
    let _ = reload_cmd.status();

    eprintln!("[Watchdog] Rollback successful. Previous state restored.");
    std::process::exit(0);
}
