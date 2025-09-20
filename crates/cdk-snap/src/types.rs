//! Types for CDK snapshot operations

use serde::{Deserialize, Serialize};
use alloy_primitives::{U256, FixedBytes, Address};
use std::collections::HashMap;

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapMetadata {
    /// Snapshot version
    pub version: u32,
    /// Creation timestamp
    pub timestamp: u64,
    /// Source database type
    pub source_type: DatabaseType,
    /// Target database type
    pub target_type: DatabaseType,
    /// Snapshot checksum
    pub checksum: String,
    /// Number of records
    pub record_count: u64,
    /// Total size in bytes
    pub total_size: u64,
}

/// Database types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseType {
    /// Reth database
    Reth,
    /// Erigon MDBX database
    ErigonMdbx,
    /// Generic snapshot format
    Snapshot,
}

/// Snapshot record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapRecord {
    /// Record key
    pub key: Vec<u8>,
    /// Record value
    pub value: Vec<u8>,
    /// Record type
    pub record_type: RecordType,
    /// Block number (if applicable)
    pub block_number: Option<U256>,
}

/// Record types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordType {
    /// Block header
    BlockHeader,
    /// Block body
    BlockBody,
    /// Transaction
    Transaction,
    /// Receipt
    Receipt,
    /// State trie node
    StateNode,
    /// Storage trie node
    StorageNode,
    /// Account
    Account,
    /// Other
    Other(String),
}

/// Conversion options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOptions {
    /// Compress output
    pub compress: bool,
    /// Compression level (1-22)
    pub compression_level: u8,
    /// Batch size for processing
    pub batch_size: usize,
    /// Validate checksums
    pub validate_checksums: bool,
    /// Progress callback interval
    pub progress_interval: u64,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            compress: true,
            compression_level: 6,
            batch_size: 1000,
            validate_checksums: true,
            progress_interval: 1000,
        }
    }
}
