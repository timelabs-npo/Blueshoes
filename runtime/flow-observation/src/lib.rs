//! Standalone observation boundary. No dependency on the edge agent or executor.
pub mod adapters;
pub mod graph;
pub mod model;
#[cfg(windows)]
#[allow(unsafe_code)]
mod win32;

pub use model::{parse_observation, Evaluation, Evidence, FlowObservation, FlowResult};
