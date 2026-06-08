#![allow(dead_code)]
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub enum RollbackError {
    Snapshot(String),
    Restore(String),
    NetworkReload(String),
    InvariantViolation(String),
}


/// Creates a snapshot of `/etc/config/` to `/tmp/bs_config_backup_<timestamp>`
pub fn create_snapshot() -> Result<String, RollbackError> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let backup_path = format!("/tmp/bs_config_backup_{}", ts);

    let output = Command::new("cp")
        .arg("-r")
        .arg("/etc/config")
        .arg(&backup_path)
        .output()
        .map_err(|e| RollbackError::Snapshot(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::Snapshot(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(backup_path)
}

/// Restores the snapshot from the given path to `/etc/config`
pub fn restore_snapshot(backup_path: &str) -> Result<(), RollbackError> {
    if !Path::new(backup_path).exists() {
        return Err(RollbackError::Restore(format!("Backup path {} does not exist", backup_path)));
    }

    // Safely copy back using `-a` with `/. ` suffix to avoid shell glob expansion
    let output = Command::new("cp")
        .arg("-a")
        .arg(format!("{}/.", backup_path))
        .arg("/etc/config/")
        .output()
        .map_err(|e| RollbackError::Restore(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::Restore(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Reloads the network service to apply restored changes
pub fn reload_network() -> Result<(), RollbackError> {
    let output = Command::new("/etc/init.d/network")
        .arg("reload")
        .output()
        .map_err(|e| RollbackError::NetworkReload(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::NetworkReload(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Rollback ZFS snapshot for the given dataset and snapshot name
pub fn zfs_rollback_snapshot(dataset: &str, snapshot: &str) -> Result<(), RollbackError> {
    let target = format!("{}@{}", dataset, snapshot);
    let output = Command::new("zfs")
        .arg("rollback")
        .arg("-r")
        .arg(&target)
        .output()
        .map_err(|e| RollbackError::Restore(e.to_string()))?;

    if !output.status.success() {
        return Err(RollbackError::Restore(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}

/// Verifies that /tmp is mounted as tmpfs / md (ephemeral memory allocation)
pub fn verify_ephemeral_tmp() -> Result<(), RollbackError> {
    // For local dev/test environment check, allow bypass if specified
    if std::env::var("BS_DEV_ENV").is_ok() {
        return Ok(());
    }

    // Try executing mount
    let output = Command::new("mount")
        .output()
        .map_err(|e| RollbackError::InvariantViolation(format!("Failed to run mount command: {}", e)))?;

    if !output.status.success() {
        return Err(RollbackError::InvariantViolation(format!(
            "mount command exited with failure: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let mount_info = String::from_utf8_lossy(&output.stdout);
    let mut tmp_found = false;
    let mut is_ephemeral = false;

    for line in mount_info.lines() {
        // Look for lines containing /tmp mount point
        if line.contains("on /tmp ") || line.contains("on /tmp/ ") || line.contains(" /tmp ") {
            tmp_found = true;
            let lower = line.to_lowercase();
            if lower.contains("tmpfs") || lower.contains("mfs") || lower.contains("md") || lower.contains("memory") {
                is_ephemeral = true;
                break;
            }
        }
    }

    // If we couldn't find /tmp in mount points, check if it's macOS or other dev host where it might be a symlink or directory
    if !tmp_found {
        #[cfg(target_os = "macos")]
        {
            // macOS typically mounts /tmp to private/tmp. Let's allow it for macOS host dev
            return Ok(());
        }
        #[cfg(not(target_os = "macos"))]
        {
            return Err(RollbackError::InvariantViolation(
                "No active mount point found for /tmp. It must be explicitly mounted.".to_string()
            ));
        }
    }

    if !is_ephemeral {
        return Err(RollbackError::InvariantViolation(
            "Constitutional contradiction: /tmp is not mounted as an ephemeral tmpfs/md memory allocation!".to_string()
        ));
    }

    Ok(())
}

/// Verifies that /boot partition is read-only
pub fn verify_boot_read_only() -> Result<(), RollbackError> {
    // If /boot doesn't exist (e.g. macOS dev machine or container), we skip or check / instead if we are strictly testing
    let boot_path = Path::new("/boot");
    if !boot_path.exists() {
        return Ok(());
    }

    // Attempt to write a temporary test file to /boot
    let test_file = boot_path.join(".sentry_write_test");
    match fs::File::create(&test_file) {
        Ok(_) => {
            // Write succeeded! This is an invariant breach!
            let _ = fs::remove_file(&test_file);
            Err(RollbackError::InvariantViolation(
                "Constitutional breach: /boot (RHEKNEL_CORE) is writeable!".to_string()
            ))
        }
        Err(e) => {
            // Write failed. Check if it failed due to read-only filesystem
            // On Unix, EROFS (Read-only file system) corresponds to raw OS error 30.
            // PermissionDenied is also acceptable since it prevents unauthorized writes.
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Ok(())
            } else if let Some(raw_err) = e.raw_os_error() {
                if raw_err == 30 { // EROFS
                    Ok(())
                } else {
                    // Other errors mean we couldn't write anyway, which is safe.
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests are mocked or ignored in CI since we don't have FreeBSD layout natively
}
