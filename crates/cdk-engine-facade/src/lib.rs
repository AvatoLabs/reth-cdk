//! Engine facade for CDK integration with Reth
//!
//! This crate provides a minimal interface for interacting with the Reth engine core.
//! It abstracts away the complexity of Reth's internal APIs and provides clean
//! interfaces for CDK operations like block import, finality marking, and rollbacks.
//!
//! The facade follows the principle of minimal integration - it only exposes
//! the essential functionality needed for CDK operations without modifying
//! the core Reth codebase.

pub mod block_import;
pub mod engine;
pub mod error;
pub mod finality;
pub mod types;
pub mod reth_integration;

pub use block_import::*;
pub use engine::*;
pub use error::*;
pub use finality::*;
pub use types::*;
pub use reth_integration::*;
