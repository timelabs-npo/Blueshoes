use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum RollbackError {
    Snapshot(String),
    Restore(String),
    NetworkReload(String),
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

#[cfg(test)]
mod tests {
    // Tests are mocked or ignored in CI since we don't have OpenWrt layout natively
}
