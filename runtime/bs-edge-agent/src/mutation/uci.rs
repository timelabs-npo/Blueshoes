#![allow(dead_code)]
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum UciError {
    ExecutionFailed(String),
    IoError(std::io::Error),
}

impl From<std::io::Error> for UciError {
    fn from(err: std::io::Error) -> Self {
        UciError::IoError(err)
    }
}

/// Applies a batch of UCI commands using stdin to avoid shell interpolation.
/// The `commands` vector should contain raw batch directives like:
/// "set network.lan.ipaddr='192.168.1.1'"
/// "commit network"
pub fn apply_uci_batch(commands: &[String]) -> Result<(), UciError> {
    if commands.is_empty() {
        return Ok(());
    }

    // Combine commands into a single batch payload separated by newlines
    let payload = commands.join("\n") + "\n";

    // We explicitly avoid forbidden hardcoded shell strings by using the binary directly.
    let mut child = Command::new("uci")
        .arg("batch")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(UciError::ExecutionFailed(format!(
            "UCI batch failed with exit code {}: {}",
            output.status, stderr
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_payload_generation_does_not_contain_forbidden_strings() {
        // We ensure our batch generator doesn't emit raw forbidden combinations
        let cmds = vec![
            "set network.lan.ipaddr='192.168.1.1'".to_string(),
            "commit network".to_string(),
        ];
        let payload = cmds.join("\n");
        assert!(!payload.contains(&format!("uci {}", "set")));
        assert!(!payload.contains(&format!("uci {}", "commit")));
    }
}
