use crate::probes::TelemetryEvent;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

fn get_journal_path() -> PathBuf {
    if cfg!(target_arch = "aarch64") && cfg!(target_env = "musl") {
        PathBuf::from("/tmp/bs-edge-journal.jsonl")
    } else {
        let dir = PathBuf::from("./target/blueshoes-dev");
        if !dir.exists() {
            std::fs::create_dir_all(&dir).unwrap_or_default();
        }
        dir.join("events.jsonl")
    }
}

pub fn append_event(event: &TelemetryEvent) -> io::Result<()> {
    append_serializable(event)
}

pub fn append_transaction(event: &crate::journal::transaction::TransactionEvent) -> io::Result<()> {
    append_serializable(event)
}

fn append_serializable<T: serde::Serialize>(event: &T) -> io::Result<()> {
    let path = get_journal_path();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let json = serde_json::to_string(event)?;
    writeln!(file, "{}", json)
}

pub fn tail_journal(lines_count: usize) -> io::Result<Vec<String>> {
    let path = get_journal_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut deque = std::collections::VecDeque::with_capacity(lines_count);

    for line in reader.lines() {
        let line = line?;
        if deque.len() == lines_count {
            deque.pop_front();
        }
        deque.push_back(line);
    }

    Ok(deque.into())
}
