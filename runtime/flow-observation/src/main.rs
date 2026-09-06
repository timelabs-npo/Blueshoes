#![forbid(unsafe_code)]
use bs_flow_observation::{
    adapters::{FlowAdapter, Gate, Win32Adapter},
    model::*,
};
use serde_json::json;

fn run() -> FlowResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 || !matches!(args[0].as_str(), "smoke" | "snapshot") {
        return Err("usage: bs-flow-observation smoke | snapshot".into());
    }
    let batch = Win32Adapter.collect()?;
    if args[0] == "snapshot" {
        println!(
            "{}",
            serde_json::to_string(&batch).map_err(|e| e.to_string())?
        );
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
            json!({"authority":Authority::ObservationOnly, "platform":"win32", "native_gate":batch.native_gate,
            "tcp_rows":batch.observations.len(), "ipv4_rows":batch.observations.len() - ipv6_rows, "ipv6_rows":ipv6_rows,
            "process_bound":process_bound, "process_unknown":batch.observations.len()-process_bound,
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
