#![forbid(unsafe_code)]
use bs_flow_observation::{
    adapters::{self, FlowAdapter, Gate, ObservationBatch},
    graph::project,
    model::*,
    parse_observation,
};
use serde_json::json;
use std::io::Read;

fn read_input(path: &str) -> FlowResult<String> {
    let mut input = String::new();
    std::fs::File::open(path)
        .map_err(|_| "cannot open input file")?
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_string(&mut input)
        .map_err(|_| "cannot read UTF-8 input")?;
    if input.len() > MAX_INPUT_BYTES {
        return Err("input exceeds 8 MiB".into());
    }
    Ok(input)
}

fn local_collect() -> FlowResult<ObservationBatch> {
    match std::env::consts::OS {
        "windows" => adapters::Win32Adapter.collect(),
        "macos" => adapters::DarwinAdapter.collect(),
        "linux" => adapters::LinuxAdapter.collect(),
        "openbsd" => adapters::OpenBsdAdapter.collect(),
        _ => Err("host native collector NOT_EXECUTED".into()),
    }
}

fn run() -> FlowResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 3 && matches!(args[0].as_str(), "fixture" | "observation") {
        let input = read_input(&args[1])?;
        let evaluation = Evaluation::new(&args[2], 30_000)?;
        let (observations, native_gate) = if args[0] == "fixture" {
            let batch = adapters::fixture(&input, &evaluation)?;
            (batch.observations, batch.native_gate)
        } else {
            (
                vec![parse_observation(&input, &evaluation)?],
                Gate::NotExecuted,
            )
        };
        println!(
            "{}",
            json!({"graph":project(&observations, &evaluation)?, "native_gate":native_gate})
        );
        return Ok(());
    }
    if args.len() != 1 || !matches!(args[0].as_str(), "smoke" | "snapshot") {
        return Err("usage: bs-flow-observation smoke | snapshot | fixture FILE RFC3339_NOW | observation FILE RFC3339_NOW".into());
    }
    let batch = local_collect()?;
    let now = chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now()).to_rfc3339();
    let graph = project(&batch.observations, &Evaluation::new(&now, 30_000)?)?;
    if args[0] == "snapshot" {
        println!("{}", json!({"batch":batch, "graph":graph}));
    } else {
        let process_bound = batch
            .observations
            .iter()
            .filter(|e| e.observation().process_ref.is_some())
            .count();
        let ipv6_rows = batch
            .observations
            .iter()
            .filter(|e| {
                e.observation()
                    .source
                    .address
                    .as_deref()
                    .is_some_and(|a| a.contains(':'))
            })
            .count();
        println!(
            "{}",
            json!({"authority":Authority::ObservationOnly, "platform":batch.platform, "native_gate":batch.native_gate,
            "tcp_rows":batch.observations.len(), "ipv4_rows":batch.observations.len() - ipv6_rows, "ipv6_rows":ipv6_rows,
            "process_bound":process_bound, "process_unknown":batch.observations.len()-process_bound,
            "graph_nodes":graph.nodes.len(), "graph_flows":graph.flows.len(),
            "collection_interval":batch.collection_interval, "gaps":batch.gaps,
            "darwin_native":"NOT_EXECUTED", "linux_native":"NOT_EXECUTED", "openbsd_native":"NOT_EXECUTED"})
        );
    }
    if batch.native_gate == Gate::NotExecuted {
        return Err("native collection NOT_EXECUTED".into());
    }
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!(
            "{}",
            json!({"status":"FAIL_OR_NOT_EXECUTED","reason":error,"authority":"observation_only"})
        );
        std::process::exit(1);
    }
}
