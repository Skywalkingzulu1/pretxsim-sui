//! PreTxSim adapter for Sui Overflow 2026 - Autonomous AI agents.
//!
//! Air-gapped structural inspection for the Sui Overflow 2026 hackathon.
//! Exposes the SuiAdapter adapter plus the shared Adapter core.

pub mod adapters;
pub use adapters::{Adapter, AdapterResult, AdapterVerdict};
pub use adapters::sui::SuiAdapter;
