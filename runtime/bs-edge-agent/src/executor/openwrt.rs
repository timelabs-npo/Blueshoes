/// DEPRECATED: This executor predates the B0 `mutation/uci.rs` batch engine.
/// It executes raw `ip`/`nft` commands directly and only supports MTU rollback.
/// For B0+ mutations, use `mutation::uci::apply_uci_batch` with `mutation::rollback`.
/// This module is retained only for the legacy M7 Canary MTU test path.
use crate::executor::{Executor, Snapshot};
use crate::journal::planner::PlanStep;
use std::process::Command;

pub struct FreeBSDExecutor;

impl Executor for FreeBSDExecutor {
    fn capture_snapshot(&self) -> std::io::Result<Snapshot> {
        // M7 Canary: Capture MTU via JSON
        let output = Command::new("ip")
            .args(["-json", "link", "show", "dev", "pppoe-wan"])
            .output()?;

        let state = String::from_utf8_lossy(&output.stdout).to_string();

        // Very basic extraction of MTU for rollback (we can just store the whole JSON for the watchdog)
        // We will just let the watchdog parse the JSON or we can parse it here.
        // Actually, for simplicity we will just extract the MTU manually or pass the JSON.
        // Let's extract the MTU.
        let parsed: serde_json::Value = serde_json::from_str(&state).unwrap_or_default();
        let mtu = parsed[0]["mtu"].as_u64().unwrap_or(1500);

        Ok(Snapshot {
            metadata: "pppoe-wan".to_string(), // ifname
            raw_state: mtu.to_string(),        // prev_mtu
        })
    }

    fn apply(&self, plan: &[PlanStep]) -> std::io::Result<()> {
        for step in plan {
            let status = match step {
                PlanStep::AddRoute { target, via } => {
                    Command::new("ip")
                        .args(["route", "add", target, "via", via])
                        .status()?
                }
                PlanStep::AddNftRule {
                    family,
                    table,
                    chain,
                    protocol,
                    dport,
                    rule_action,
                } => {
                    use crate::journal::planner::{
                        NftAction, NftChain, NftFamily, NftTable, TransportProtocol,
                    };

                    let family_str = match family {
                        NftFamily::Inet => "inet",
                    };
                    let table_str = match table {
                        NftTable::Filter => "filter",
                    };
                    let chain_str = match chain {
                        NftChain::Forward => "forward",
                    };
                    let proto_str = match protocol {
                        TransportProtocol::Tcp => "tcp",
                        TransportProtocol::Udp => "udp",
                    };
                    let action_str = match rule_action {
                        NftAction::Accept => "accept",
                        NftAction::Drop => "drop",
                        NftAction::Reject => "reject",
                    };
                    Command::new("nft")
                        .args([
                            "add",
                            "rule",
                            family_str,
                            table_str,
                            chain_str,
                            proto_str,
                            "dport",
                            &dport.to_string(),
                            action_str,
                        ])
                        .status()?
                }
                PlanStep::SetMtu { interface, mtu } => {
                    Command::new("ip")
                        .args(["link", "set", "dev", interface, "mtu", &mtu.to_string()])
                        .status()?
                }
                PlanStep::FlushRouteCache => {
                    Command::new("ip")
                        .args(["route", "flush", "cache"])
                        .status()?
                }
            };

            if !status.success() {
                return Err(std::io::Error::other(
                    format!("Command for step {:?} failed with exit code: {}", step, status),
                ));
            }
        }
        Ok(())
    }

    fn rollback(&self, snapshot: &Snapshot) -> std::io::Result<()> {
        let _ = Command::new("ip")
            .args([
                "link",
                "set",
                "dev",
                &snapshot.metadata,
                "mtu",
                &snapshot.raw_state,
            ])
            .status()?;
        Ok(())
    }
}
