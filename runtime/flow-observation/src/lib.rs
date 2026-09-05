//! Standalone observation boundary. No dependency on the edge agent or executor.
pub mod adapters;
pub mod model;

pub use model::{parse_observation, Evaluation, Evidence, FlowObservation, FlowResult};
