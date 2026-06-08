use std::env;
use std::fs::{self, File};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::path::Path;
use tar::Archive;
use flate2::read::GzDecoder;

fn verify_ephemeral_tmp() -> Result<(), String> {
    if env::var("BS_DEV_ENV").is_ok() {
        return Ok(());
    }

    let output = Command::new("mount")
        .output()
        .map_err(|e| format!("Failed to run mount command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "mount command exited with failure: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let mount_info = String::from_utf8_lossy(&output.stdout);
    let mut tmp_found = false;
    let mut is_ephemeral = false;

    for line in mount_info.lines() {
        if line.contains("on /tmp ") || line.contains("on /tmp/ ") || line.contains(" /tmp ") {
            tmp_found = true;
            let lower = line.to_lowercase();
            if lower.contains("tmpfs") || lower.contains("mfs") || lower.contains("md") || lower.contains("memory") {
                is_ephemeral = true;
                break;
            }
        }
    }

    if !tmp_found {
        #[cfg(target_os = "macos")]
        {
            return Ok(());
        }
        #[cfg(not(target_os = "macos"))]
        {
            return Err("No active mount point found for /tmp.".to_string());
        }
    }

    if !is_ephemeral {
        return Err("Constitutional contradiction: /tmp is not mounted as an ephemeral tmpfs/md memory allocation!".to_string());
    }

    Ok(())
}

fn verify_boot_read_only() -> Result<(), String> {
    let boot_path = Path::new("/boot");
    if !boot_path.exists() {
        return Ok(());
    }

    let test_file = boot_path.join(".sentry_write_test");
    match File::create(&test_file) {
        Ok(_) => {
            let _ = fs::remove_file(&test_file);
            Err("Constitutional breach: /boot (RHEKNEL_CORE) is writeable!".to_string())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Ok(())
            } else if let Some(raw_err) = e.raw_os_error() {
                if raw_err == 30 { // EROFS
                    Ok(())
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
    }
}

fn perform_zfs_rollback(dataset_snapshot: &str) -> Result<(), String> {
    println!("[Watchdog] Performing ZFS rollback to snapshot {}...", dataset_snapshot);
    let output = Command::new("zfs")
        .arg("rollback")
        .arg("-r")
        .arg(dataset_snapshot)
        .output()
        .map_err(|e| format!("Failed to run zfs rollback: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "zfs rollback failed with status {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

fn trigger_panic_and_rollback(reason: &str) -> ! {
    eprintln!("[Watchdog] PANIC: INVARIANT VIOLATION: {}", reason);
    eprintln!("[Watchdog] Initiating emergency rollback...");
    
    let use_zfs = env::var("BS_USE_ZFS").unwrap_or_else(|_| "0".to_string()) == "1"
        || Command::new("zfs").arg("list").output().is_ok();

    if use_zfs {
        let zfs_target = env::var("BS_ZFS_SNAPSHOT").unwrap_or_else(|_| "tank/blueshoes/runtime@genesis_stable".to_string());
        match perform_zfs_rollback(&zfs_target) {
            Ok(_) => eprintln!("[Watchdog] Emergency ZFS rollback completed successfully."),
            Err(e) => eprintln!("[Watchdog] EMERGENCY ROLLBACK FAILED: {}", e),
        }
    } else {
        eprintln!("[Watchdog] ZFS not available. Performing standard rollback fallback...");
        let rollback_file = "/tmp/blueshoes/rollbacks/rb_1.tar.gz";
        if Path::new(rollback_file).exists() {
            if let Ok(f) = File::open(rollback_file) {
                let dec = GzDecoder::new(f);
                let mut archive = Archive::new(dec);
                let target_dir = if Path::new("/etc/config").exists() { "/etc" } else { "/tmp" };
                if let Err(e) = archive.unpack(target_dir) {
                    eprintln!("[Watchdog] EMERGENCY ROLLBACK FAILED: Failed to unpack: {}", e);
                } else {
                    eprintln!("[Watchdog] Emergency standard rollback completed successfully.");
                }
            }
        }
    }
    std::process::exit(101);
}

fn run_compliance_checks() {
    if let Err(e) = verify_ephemeral_tmp() {
        trigger_panic_and_rollback(&format!("Ephemeral /tmp check failed: {}", e));
    }
    if let Err(e) = verify_boot_read_only() {
        trigger_panic_and_rollback(&format!("Read-only /boot check failed: {}", e));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tx_id = String::new();
    let mut timeout_secs = 60;
    
    let mut is_legacy = false;
    let mut legacy_ifname = String::new();
    let mut legacy_mtu = String::new();

    if args.len() == 4 && !args[1].starts_with("-") {
        is_legacy = true;
        legacy_ifname = args[1].clone();
        legacy_mtu = args[2].clone();
        timeout_secs = args[3].parse().unwrap_or(30);
    } else {
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
            eprintln!("Usage:");
            eprintln!("  New:    bs-watchdog --tx-id <id> [--timeout <secs>]");
            eprintln!("  Legacy: bs-watchdog <ifname> <prev_mtu> <timeout_secs>");
            std::process::exit(1);
        }
    }

    run_compliance_checks();

    if is_legacy {
        thread::spawn(move || {
            use std::io::BufRead;
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(l) = line {
                    if l.trim() == "COMMIT" {
                        println!("[Watchdog] Legacy COMMIT received via stdin. Disarming.");
                        std::process::exit(0);
                    }
                }
            }
        });
        println!("[Watchdog] Armed in legacy mode for {} (prev MTU: {}) with timeout {}s", legacy_ifname, legacy_mtu, timeout_secs);
    } else {
        println!("[Watchdog] Armed for TX {} with timeout {}s", tx_id, timeout_secs);
    }

    let confirm_file = if is_legacy {
        format!("/tmp/blueshoes_confirm_canary")
    } else {
        format!("/tmp/blueshoes_confirm_{}", tx_id)
    };

    let check_interval = Duration::from_secs(1);
    let mut elapsed = 0;

    while elapsed < timeout_secs {
        run_compliance_checks();

        if Path::new(&confirm_file).exists() {
            println!("[Watchdog] Commit confirmed! Disarming watchdog.");
            let _ = fs::remove_file(&confirm_file);
            std::process::exit(0);
        }
        thread::sleep(check_interval);
        elapsed += 1;
    }

    eprintln!("\n[Watchdog] CRITICAL: Timeout ({}s) expired! Triggering dead-man's switch rollback...", timeout_secs);

    let use_zfs = env::var("BS_USE_ZFS").unwrap_or_else(|_| "0".to_string()) == "1"
        || Command::new("zfs").arg("list").output().is_ok();

    if use_zfs {
        let zfs_target = env::var("BS_ZFS_SNAPSHOT").unwrap_or_else(|_| "tank/blueshoes/runtime@genesis_stable".to_string());
        match perform_zfs_rollback(&zfs_target) {
            Ok(_) => {
                println!("[Watchdog] Rollback successful via ZFS.");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("[Watchdog] ZFS Rollback failed: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        if is_legacy {
            let _ = Command::new("ip")
                .args([
                    "link",
                    "set",
                    "dev",
                    &legacy_ifname,
                    "mtu",
                    &legacy_mtu,
                ])
                .status();
            println!("[Watchdog] Legacy MTU restored.");
            std::process::exit(0);
        } else {
            let rollback_file = "/tmp/blueshoes/rollbacks/rb_1.tar.gz";
            if !Path::new(rollback_file).exists() {
                eprintln!("[Watchdog] ERROR: Rollback archive {} not found!", rollback_file);
                std::process::exit(1);
            }

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
                "/tmp"
            };

            if let Err(e) = archive.unpack(target_dir) {
                eprintln!("[Watchdog] Failed to unpack rollback archive: {}", e);
                std::process::exit(1);
            }

            println!("[Watchdog] Configuration restored. Restarting network services...");
            let mut reload_cmd = Command::new("/etc/init.d/network");
            reload_cmd.arg("restart");
            let _ = reload_cmd.status();

            eprintln!("[Watchdog] Rollback successful. Previous state restored.");
            std::process::exit(0);
        }
    }
}
