// linter:exempt
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn test_no_mutating_commands_in_source() {
    let forbidden_strings = vec![
        "uci set",
        "uci commit",
        "nft add",
        "nft delete",
        "iptables",
        "ip route add",
        "ip route del",
        "wg set",
    ];

    let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut found_violations = Vec::new();

    for entry in WalkDir::new(&src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // Skip build artifacts and hidden directories
        if path.components().any(|c| {
            let s = c.as_os_str().to_string_lossy();
            s == "target" || s == "artifacts" || s.starts_with('.')
        }) {
            continue;
        }

        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    let content = fs::read_to_string(entry.path()).expect("Failed to read file");

                    if content.contains("linter:exempt") {
                        continue;
                    }

                    for (line_num, line) in content.lines().enumerate() {
                        for forbidden in &forbidden_strings {
                            if line.contains(forbidden) {
                                // Allow comments that discuss the forbidden strings
                                let trimmed = line.trim_start();
                                if !trimmed.starts_with("//") && !line.contains("linter:exempt") {
                                    found_violations.push(format!(
                                        "File: {}, Line {}: contains forbidden string '{}'",
                                        entry.path().display(),
                                        line_num + 1,
                                        forbidden
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !found_violations.is_empty() {
        panic!(
            "M1 Audit Failed! Mutating commands found in source code:\n{}",
            found_violations.join("\n")
        );
    }
}

#[test]
fn test_cli_default_safety() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bs-edge-agent"))
        .arg("canary")
        .output()
        .expect("Failed to execute bs-edge-agent");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"execution_mode\": \"dry_run\""));
    assert!(stdout.contains("\"mutation_performed\": false"));
    assert!(
        stdout.contains("dangerous_execution feature disabled")
            || stdout.contains("missing --unsafe-execute flag")
    );
}

#[test]
fn test_cli_force_dry_run() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_bs-edge-agent"))
        .arg("--unsafe-execute")
        .arg("--confirm")
        .arg("unsafe:test")
        .arg("canary")
        .env("BS_FORCE_DRY_RUN", "1")
        .output()
        .expect("Failed to execute bs-edge-agent");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"execution_mode\": \"dry_run\""));
    assert!(stdout.contains("BS_FORCE_DRY_RUN=1 environment variable set"));
}
