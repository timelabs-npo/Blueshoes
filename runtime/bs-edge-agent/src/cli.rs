use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(long, global = true, default_value_t = false)]
    pub unsafe_execute: bool,

    #[arg(long, global = true)]
    pub confirm: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check the status of the router (memory, load, OS)
    Status,
    /// Perform a read-only network validation check and append to journal
    Netcheck,
    /// List available static routing profiles
    Profiles,
    /// Output the local event journal
    Journal {
        #[arg(long)]
        tail: Option<usize>,
    },
    /// Run a quick self-diagnostic
    Doctor,
    /// Dump safe environment variables
    Env,
    /// Perform an explicit DNS lookup
    Dns {
        target: String,
    },
    /// Perform an explicit ICMP ping (latency)
    Latency {
        #[arg(required = true)]
        target: String,
    },
    /// Perform a trace to a target
    Trace {
        #[arg(required = true)]
        target: String,
    },


    /// Run M7 Canary Mutation validation
    Canary,
    /// Collect router facts for the MECHA harness
    Facts,
    /// Generate a candidate capability graph and save to file (dry-run)
    Plan {
        #[arg(required = true)]
        profile: String,
        #[arg(long, required = true)]
        out: String,
    },
    /// Activate a candidate configuration and arm the rollback watchdog
    ApplyConfirmed {
        #[arg(required = true)]
        plan_file: String,
        #[arg(long, default_value_t = 60)]
        timeout: u64,
    },
    /// Disarm the watchdog, making the configuration permanent
    Confirm {
        #[arg(required = true)]
        tx_id: String,
    },
    /// Export the semantic substrate as JSON-LD
    SubstrateExport {
        /// Optional output file path. Prints to stdout if omitted.
        #[arg(long)]
        out: Option<String>,
    },
    /// Verify the provenance chain integrity of the substrate
    SubstrateVerify,
    /// Run mathematical drift analysis against an agent's output tokens
    DriftAudit {
        /// Path to a JSON file containing an array of concept type strings
        #[arg(required = true)]
        payload_file: String,
        /// KL divergence alarm threshold in bits
        #[arg(long, default_value_t = 0.35)]
        threshold: f64,
    },
}
