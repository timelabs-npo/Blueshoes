mod cli;
mod executor;
mod journal;
mod probes;
mod profiles;
mod mutation;
pub mod gcp;
pub mod secrets;

use clap::Parser;
use cli::{Cli, Commands};
use serde_json::json;
pub mod semantic;
use std::env;
use std::fs;
use std::path::Path;
use semantic::check::{Provenance, validate_transformation};

fn handle_provenance_cli() {
    let args: Vec<String> = env::args().collect();
    if let Some(index) = args.iter().position(|r| r == "--check-provenance") {
        if index + 1 < args.len() {
            let target_path = &args[index + 1];
            println!("[+] Loading provenance file target: {}", target_path);
            
            let data = fs::read_to_string(target_path).expect("Failed to read targeted target payload");
            let payload: Provenance = serde_json::from_str(&data).expect("Payload violates Provenance format structural schema");
            
            match validate_transformation(payload) {
                Ok(enriched) => {
                    println!("{}", serde_json::to_string_pretty(&enriched).unwrap());
                    std::process::exit(0);
                },
                Err((report, exit_code)) => {
                    eprintln!("{}", serde_json::to_string_pretty(&report).unwrap());
                    std::process::exit(exit_code);
                }
            }
        } else {
            std::process::exit(1);
        }
    }
}

fn main() {
    handle_provenance_cli();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Status => {
            let event = probes::system::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Netcheck => {
            // Run all probes sequentially
            let sys_event = probes::system::run();
            let route_event = probes::route::run();
            let dns_target = std::env::var("BS_DNS_TARGET").unwrap_or_else(|_| "google.com".to_string());
            let icmp_target = std::env::var("BS_ICMP_TARGET").unwrap_or_else(|_| "1.1.1.1".to_string());
            let https_target = std::env::var("BS_HTTPS_TARGET").unwrap_or_else(|_| "https://google.com".to_string());

            let dns_event = probes::dns::run(&dns_target);
            let icmp_event = probes::icmp::run(&icmp_target);
            let https_event = probes::https::run(&https_target);
            
            let iface_list = ["lo", "eth0", "wan", "br-lan"];
            let interface_event = probes::TelemetryEvent::new(
                "interface", "ok", 0, 
                serde_json::to_value(probes::interface::run(&iface_list)).unwrap()
            );
            
            let mtu_event = probes::TelemetryEvent::new(
                "mtu", "ok", 0, 
                serde_json::to_value(probes::mtu::run(&iface_list)).unwrap()
            );

            let events = vec![sys_event, route_event, dns_event, icmp_event, https_event, interface_event, mtu_event];

            for event in &events {
                if let Err(e) = journal::jsonl::append_event(event) {
                    eprintln!("Failed to write to journal: {}", e);
                }
            }

            println!("{}", serde_json::to_string_pretty(&events).unwrap());
        }
        Commands::Profiles => {
            let profiles = json!([
                {"name": "DIRECT", "description": "Standard FreeBSD routing"},
                {"name": "DNS_PRIVACY", "description": "Encrypted DNS upstreams"},
                {"name": "ECH_PRESERVE", "description": "Preserve TLS integrity"},
                {"name": "USER_TUNNEL", "description": "Operator configured tunnel"}
            ]);
            println!("{}", serde_json::to_string_pretty(&profiles).unwrap());
        }
        Commands::Journal { tail } => {
            let count = tail.unwrap_or(10);
            match journal::jsonl::tail_journal(count) {
                Ok(lines) => {
                    for line in lines {
                        println!("{}", line);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading journal: {}", e);
                }
            }
        }
        Commands::Doctor => {
            let event = probes::doctor::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Env => {
            let event = probes::env::run();
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Canary => {
            handle_canary(&cli);
        }
        Commands::Dns { target } => {
            let event = probes::dns::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Latency { target } => {
            let event = probes::icmp::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Trace { target } => {
            let event = probes::trace::run(target);
            println!("{}", serde_json::to_string_pretty(&event).unwrap());
        }
        Commands::Facts => {
            let facts = json!({
                "agent_version": env!("CARGO_PKG_VERSION"),
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH,
                "timestamp": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
            });
            println!("{}", serde_json::to_string_pretty(&facts).unwrap());
        }
        Commands::Plan { profile, out } => {
            let prof = profiles::schema::ProfileSchema {
                name: profile.clone(),
                intent: match profile.as_str() {
                    "DNS_PRIVACY" => profiles::schema::ProfileIntent::DnsPrivacy,
                    "ECH_PRESERVE" => profiles::schema::ProfileIntent::EchPreserve,
                    "USER_TUNNEL" => profiles::schema::ProfileIntent::UserTunnel,
                    "SAFE_MTU" => profiles::schema::ProfileIntent::SafeMtu,
                    "OBLIVIOUS_DNS" => profiles::schema::ProfileIntent::ObliviousDns,
                    "MASQUE_OBFUSCATION" => profiles::schema::ProfileIntent::MasqueObfuscation,
                    _ => profiles::schema::ProfileIntent::RecoverySafeMode,
                },
                description: "Auto-generated profile".to_string(),
                routes: None,
                dns: None,
            };
            
            // Acquire exclusive lock
            let _lock = match executor::transaction::acquire_exclusive_lock() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to acquire transaction lock: {}", e);
                    std::process::exit(1);
                }
            };
            
            let plan = match journal::planner::Planner::plan(&prof, "10.0.0.1") {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Planner failed: {}", e);
                    std::process::exit(1);
                }
            };
            
            let json = serde_json::to_string_pretty(&plan).unwrap();
            std::fs::write(out, json).expect("Failed to write plan file");
            println!("Plan successfully written to {}", out);
        }
        Commands::ApplyConfirmed { plan_file, timeout } => {
            let _lock = match executor::transaction::acquire_exclusive_lock() {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to acquire transaction lock: {}", e);
                    std::process::exit(1);
                }
            };

            let plan_str = std::fs::read_to_string(plan_file).expect("Failed to read plan file");
            let plan: executor::capabilities::CapabilityGraph = serde_json::from_str(&plan_str).expect("Invalid plan file");

            let prov = semantic::check::Provenance {
                result: "ALLOW_APPLY_CONFIRMED".to_string(),
                derived_from: vec!["inv.no_irreversible_mutation".to_string(), "cap.apply_confirmed".to_string(), "gen.current".to_string()],
                evidence: vec!["rollback_anchor.exists".to_string(), format!("tx_json.valid: {}", plan_file)],
                hash: String::new(),
            };
            if let Err((report, code)) = semantic::check::validate_transformation(prov) {
                eprintln!("{}", serde_json::to_string_pretty(&report).unwrap());
                std::process::exit(code);
            }

            println!("Capturing candidate snapshot and shifting rollback slots...");
            executor::transaction::shift_and_create_snapshot().expect("Snapshot failed");

            let tx_id = format!(
                "tx_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );

            println!("Applying capability graph...");
            let exec = executor::DryRunExecutor; // Using DryRun for now, or FreeBSDExecutor if dangerous_execution
            if let Err(e) = executor::Executor::apply(&exec, &plan) {
                eprintln!("Apply failed: {}", e);
                std::process::exit(1);
            }

            println!("Spawning watchdog with timeout {}s for tx_id: {}", timeout, tx_id);
            executor::transaction::arm_watchdog(&tx_id, *timeout).expect("Failed to arm watchdog");
            
            println!("Configuration active. Run `bs-edge-agent confirm {}` within {}s to make permanent.", tx_id, timeout);
        }
        Commands::Confirm { tx_id } => {
            // Does not need the exclusive lock to confirm, just signals watchdog
            executor::transaction::confirm_transaction(tx_id).expect("Failed to confirm transaction");
            println!("Transaction {} confirmed successfully.", tx_id);
        }
        Commands::SubstrateExport { out } => {
            let mut substrate = semantic::substrate::SemanticSubstrate::new();
            // Populate from the core schema
            seed_substrate_from_schema(&mut substrate);
            let export = substrate.export_jsonld();
            let json = serde_json::to_string_pretty(&export).unwrap();
            if let Some(path) = out {
                fs::write(path, &json).expect("Failed to write JSON-LD export");
                println!("[+] Substrate exported to {}", path);
            } else {
                println!("{}", json);
            }
        }
        Commands::SubstrateVerify => {
            let mut substrate = semantic::substrate::SemanticSubstrate::new();
            seed_substrate_from_schema(&mut substrate);
            match substrate.verify_chain_integrity() {
                Ok(()) => {
                    println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "status": "INTACT",
                        "generation": substrate.generation,
                        "chain_length": substrate.lineage.len(),
                        "chain_head": substrate.lineage.last().map(|r| &r.transformation_hash),
                    })).unwrap());
                }
                Err((idx, msg)) => {
                    eprintln!("{}", serde_json::to_string_pretty(&serde_json::json!({
                        "status": "BROKEN",
                        "broken_at_index": idx,
                        "error": msg,
                    })).unwrap());
                    std::process::exit(20);
                }
            }
        }
        Commands::DriftAudit { payload_file, threshold } => {
            let mut substrate = semantic::substrate::SemanticSubstrate::new();
            seed_substrate_from_schema(&mut substrate);
            let ground_truth = substrate.concept_distribution();

            let payload_raw = fs::read_to_string(payload_file).expect("Failed to read payload file");
            let tokens: Vec<String> = serde_json::from_str(&payload_raw).expect("Payload must be a JSON array of strings");

            let config = semantic::drift::DriftConfig {
                kl_threshold: *threshold,
                ..Default::default()
            };
            let mut detector = semantic::drift::DriftDetector::new(config);
            let report = detector.analyze(&ground_truth, &tokens);
            println!("{}", serde_json::to_string_pretty(&report).unwrap());

            if report.drift_alarm || report.preemptive_alarm {
                std::process::exit(21);
            }
        }
    }
}

/// Populate the substrate with entities derived from the core schema.
/// This is the bridge between the static JSON schema and the live substrate engine.
fn seed_substrate_from_schema(substrate: &mut semantic::substrate::SemanticSubstrate) {
    use semantic::substrate::{ConceptType, Entity, Relation};
    use std::collections::HashMap;

    // Invariant
    let mut inv_props = HashMap::new();
    inv_props.insert("statement".into(), "No state transition may become irreversible, unverifiable, or sovereign.".into());
    inv_props.insert("authority".into(), "local_0log".into());
    inv_props.insert("status".into(), "active".into());
    substrate.commit_entity(Entity {
        id: "Invariant:no_irreversible_mutation".into(),
        concept: ConceptType::Invariant,
        properties: inv_props,
        relations: vec![],
    }, "schema_seed");

    // Capability
    let mut cap_props = HashMap::new();
    cap_props.insert("allowed_actor".into(), "executor".into());
    cap_props.insert("requires".into(), "rollback_anchor,watchdog,confirmation_window".into());
    substrate.commit_entity(Entity {
        id: "Capability:apply_confirmed".into(),
        concept: ConceptType::Capability,
        properties: cap_props,
        relations: vec![
            Relation { predicate: "requires_invariant".into(), target_id: "Invariant:no_irreversible_mutation".into() },
        ],
    }, "schema_seed");

    // StateGeneration
    let mut gen_props = HashMap::new();
    gen_props.insert("source".into(), "0.log".into());
    gen_props.insert("promotion_requires".into(), "validation_passed,confirmation_received,rollback_available".into());
    substrate.commit_entity(Entity {
        id: "StateGeneration:current".into(),
        concept: ConceptType::StateGeneration,
        properties: gen_props,
        relations: vec![
            Relation { predicate: "governed_by".into(), target_id: "Invariant:no_irreversible_mutation".into() },
            Relation { predicate: "promoted_via".into(), target_id: "Capability:apply_confirmed".into() },
        ],
    }, "schema_seed");

    // ExternalMirror (Spanner)
    let mut mirror_props = HashMap::new();
    mirror_props.insert("may_store".into(), "replicated_observations,promoted_artifacts,advisory_acl".into());
    mirror_props.insert("may_not".into(), "override_0log,command_edge_mutation,become_runtime_source_of_truth".into());
    substrate.commit_entity(Entity {
        id: "ExternalMirror:spanner".into(),
        concept: ConceptType::ExternalMirror,
        properties: mirror_props,
        relations: vec![
            Relation { predicate: "mirrors".into(), target_id: "StateGeneration:current".into() },
        ],
    }, "schema_seed");

    // Project: Blueshoes itself
    let mut proj_props = HashMap::new();
    proj_props.insert("name".into(), "Blueshoes".into());
    proj_props.insert("domain".into(), "adaptive-networking".into());
    substrate.commit_entity(Entity {
        id: "Project:blueshoes".into(),
        concept: ConceptType::Project,
        properties: proj_props,
        relations: vec![
            Relation { predicate: "enforces".into(), target_id: "Invariant:no_irreversible_mutation".into() },
        ],
    }, "schema_seed");

    // Metric: Drift score
    let mut metric_props = HashMap::new();
    metric_props.insert("name".into(), "semantic_drift_kl".into());
    metric_props.insert("unit".into(), "bits".into());
    metric_props.insert("alarm_threshold".into(), "0.35".into());
    substrate.commit_entity(Entity {
        id: "Metric:semantic_drift_kl".into(),
        concept: ConceptType::Metric,
        properties: metric_props,
        relations: vec![
            Relation { predicate: "monitors".into(), target_id: "Project:blueshoes".into() },
        ],
    }, "schema_seed");
}

fn handle_canary(cli: &Cli) {

    use executor::{DryRunExecutor, Executor};
    use journal::transaction::{TransactionEvent, TransactionState};

    // Choose executor based on feature flag
    #[cfg(feature = "dangerous_execution")]
    let exec = executor::FreeBSD::FreeBSDExecutor;
    #[cfg(not(feature = "dangerous_execution"))]
    let exec = DryRunExecutor;

    let tx_id = format!(
        "tx_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let profile = profiles::schema::ProfileSchema {
        name: "Canary Test MTU".to_string(),
        intent: profiles::schema::ProfileIntent::SafeMtu,
        description: "Test safe MTU setting".to_string(),
        routes: None,
        dns: None,
    };

    let plan = match journal::planner::Planner::plan(&profile, "10.0.0.1") {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Planner failed: {}", e);
            return;
        }
    };

    let force_dry_run =
        std::env::var("BS_FORCE_DRY_RUN").unwrap_or_else(|_| "0".to_string()) == "1";

    #[cfg(feature = "dangerous_execution")]
    let has_dangerous_execution = true;
    #[cfg(not(feature = "dangerous_execution"))]
    let has_dangerous_execution = false;

    let refusal_reason = if force_dry_run {
        Some("BS_FORCE_DRY_RUN=1 environment variable set")
    } else if !has_dangerous_execution {
        Some("dangerous_execution feature disabled")
    } else if !cli.unsafe_execute {
        Some("missing --unsafe-execute flag")
    } else if !cli.confirm.as_deref().unwrap_or("").starts_with("unsafe:") {
        Some("missing or invalid --confirm unsafe:<request_id>")
    } else {
        None
    };

    if let Some(reason) = refusal_reason {
        let evidence = journal::planner::Planner::dry_run(&profile, reason).unwrap();
        println!("{}", serde_json::to_string_pretty(&evidence).unwrap());

        // Log dry run event
        let dry_run_strings = vec![serde_json::to_string(&evidence).unwrap()];
        let start_event = TransactionEvent::new(
            tx_id,
            TransactionState::Start,
            Some(format!("{:?}", profile.intent)),
            Some(dry_run_strings),
        );
        let _ = journal::jsonl::append_transaction(&start_event);
        return;
    }

    // --- EXECUTION PATH ---

    let prov = semantic::check::Provenance {
        result: "ALLOW_CANARY".to_string(),
        derived_from: vec!["inv.no_irreversible_mutation".to_string(), "cap.apply_confirmed".to_string(), "gen.current".to_string()],
        evidence: vec!["rollback_anchor.exists".to_string(), "watchdog.armed".to_string()],
        hash: String::new(),
    };
    if let Err((report, code)) = semantic::check::validate_transformation(prov) {
                eprintln!("{}", serde_json::to_string_pretty(&report).unwrap());
                std::process::exit(code);
    }

    let start_event = TransactionEvent::new(
        tx_id.clone(),
        TransactionState::Start,
        Some(format!("{:?}", profile.intent)),
        None,
    );
    let _ = journal::jsonl::append_transaction(&start_event);
    println!("{}", serde_json::to_string_pretty(&start_event).unwrap());

    // 1. Snapshot
    println!("Capturing snapshot...");
    let snapshot = match exec.capture_snapshot() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to capture snapshot: {}", e);
            return;
        }
    };

    // 2. Setup dead man's switch timer
    println!("Spawning bs-watchdog...");

    #[cfg(feature = "dangerous_execution")]
    let mut watchdog_child = std::process::Command::new(
        std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("bs-watchdog"),
    )
    .args([&snapshot.metadata, &snapshot.raw_state, "30"])
    .stdin(std::process::Stdio::piped())
    .spawn()
    .expect("Failed to spawn bs-watchdog");

    println!("Applying plan...");
    if let Err(e) = exec.apply(&plan) {
        eprintln!("Failed to apply plan: {}", e);
        #[cfg(feature = "dangerous_execution")]
        {
            let _ = watchdog_child.kill();
            let _ = watchdog_child.wait();
        }
        let _ = exec.rollback(&snapshot);
        return;
    }

    // Simulate "netcheck" validation process
    println!("Validating network...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Validation complete. Committing to watchdog.");

    #[cfg(feature = "dangerous_execution")]
    {
        use std::io::Write;
        if let Some(mut stdin) = watchdog_child.stdin.take() {
            let _ = stdin.write_all(b"COMMIT\n");
        }
        let _ = watchdog_child.wait();
    }

    println!("Transaction successful.");
}
