pub mod capabilities;
pub mod masque;
pub mod transaction;

use crate::executor::capabilities::CapabilityGraph;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub metadata: String,
    pub raw_state: String,
}

pub trait Executor {
    fn capture_snapshot(&self) -> std::io::Result<Snapshot>;
    fn apply(&self, plan: &CapabilityGraph) -> std::io::Result<()>;
    fn rollback(&self, snapshot: &Snapshot) -> std::io::Result<()>;
}

pub struct DryRunExecutor;

impl Executor for DryRunExecutor {
    fn capture_snapshot(&self) -> std::io::Result<Snapshot> {
        println!("[DryRun] Capturing safe snapshot...");
        Ok(Snapshot {
            metadata: "DryRun Default".to_string(),
            raw_state: "mock_state".to_string(),
        })
    }

    fn apply(&self, plan: &CapabilityGraph) -> std::io::Result<()> {
        println!("[DryRun] Applying plan safely (zero mutation):");
        for step in &plan.network_caps {
            println!("  -> {:?}", step);
        }
        Ok(())
    }

    fn rollback(&self, _snapshot: &Snapshot) -> std::io::Result<()> {
        println!("[DryRun] Rolling back to snapshot safely...");
        Ok(())
    }
}

pub mod freebsd;
pub use freebsd::FreeBsdExecutor;

#[cfg(feature = "dangerous_execution")]
pub mod freebsd_legacy;
#[cfg(feature = "dangerous_execution")]
pub use freebsd_legacy as FreeBSD;
