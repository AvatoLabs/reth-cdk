//! CDK RPC Extensions
//!
//! This crate provides extended RPC methods for querying CDK-specific data structures
//! like batches, epochs, and finality information.

pub mod api;
pub mod error;
pub mod server;
pub mod types;

pub use api::{CdkRpcApi, CdkRpcApiImpl};
pub use error::{CdkRpcError, CdkRpcResult};
pub use server::{CdkRpcConfig, CdkRpcServer};
pub use types::*;

/// Re-export commonly used types
pub use cdk_types::{Batch, BatchId, Epoch, EpochId, FinalityTag};
pub use alloy_primitives::U256;
