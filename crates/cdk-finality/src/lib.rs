//! Finality oracle for CDK L1 finality checking
//!
//! This crate provides the `FinalityOracle` trait and implementations for
//! checking L1 finality status and managing rollbacks. It reads from L1
//! contracts to determine when batches can be considered final.

pub mod error;
pub mod oracle;
pub mod l1_client;
pub mod rollback;
pub mod l1_contract;

pub use error::*;
pub use oracle::*;
pub use l1_client::*;
pub use rollback::*;
pub use l1_contract::*;
