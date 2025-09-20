//! Core types for CDK (Celestia Data Availability) integration with Reth
//!
//! This crate defines the fundamental data structures used throughout the CDK integration:
//! - `Batch`: Represents a batch of blocks with L1 origin information
//! - `Epoch`: Represents a time period with start/end block boundaries  
//! - `FinalityTag`: Represents finality status from L1 contracts
//!
//! All types use `alloy-primitives` for consistent Ethereum primitive handling.

pub mod batch;
pub mod epoch;
pub mod finality;
pub mod error;

pub use batch::*;
pub use epoch::*;
pub use finality::*;
pub use error::*;