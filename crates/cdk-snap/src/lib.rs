//! CDK Snapshot and Index Interoperability Tools
//!
//! This module provides tools for snapshot and index interoperability between
//! Reth and Erigon MDBX databases, enabling data migration and validation.

pub mod converter;
pub mod validator;
pub mod error;
pub mod types;

pub use error::{SnapError, SnapResult};
pub use types::*;

/// CDK Snapshot version
pub const CDK_SNAP_VERSION: u32 = 1;

/// CDK Snapshot magic bytes
pub const CDK_SNAP_MAGIC: &[u8] = b"CDK_SNAP_V1";
