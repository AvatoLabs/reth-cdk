//! RPC request and response types

use alloy_primitives::U256;
use serde::{Deserialize, Serialize};
use cdk_types::{Batch, BatchId, Epoch};

/// Request to get batch by number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBatchByNumberRequest {
    /// Batch number (hex string)
    pub batch_number: String,
}

/// Request to get epoch by block number
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEpochByBlockRequest {
    /// Block number (hex string)
    pub block_number: String,
}

/// CDK metrics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdkMetrics {
    /// Total number of batches processed
    pub total_batches: u64,
    /// Total number of epochs processed
    pub total_epochs: u64,
    /// Latest batch number
    pub latest_batch: Option<U256>,
    /// Latest epoch number
    pub latest_epoch: Option<U256>,
    /// Latest finalized batch
    pub latest_finalized_batch: Option<U256>,
    /// L1 lag (blocks behind)
    pub l1_lag: Option<u64>,
    /// Reorganization count
    pub reorg_count: u64,
    /// Ingest TPS (transactions per second)
    pub ingest_tps: f64,
}

/// Finalized batch response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalizedBatchResponse {
    /// Finalized batch ID
    pub batch_id: BatchId,
    /// Finality status
    pub status: String,
    /// L1 block number
    pub l1_block: U256,
    /// Finality timestamp
    pub timestamp: u64,
}

/// Batch response with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse {
    /// The batch data
    pub batch: Batch,
    /// Additional metadata
    pub metadata: BatchMetadata,
}

/// Epoch response with additional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochResponse {
    /// The epoch data
    pub epoch: Epoch,
    /// Additional metadata
    pub metadata: EpochMetadata,
}

/// Batch metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetadata {
    /// Number of blocks in this batch
    pub block_count: u64,
    /// Total transactions in this batch
    pub transaction_count: u64,
    /// Batch size in bytes
    pub size_bytes: u64,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// Epoch metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochMetadata {
    /// Number of batches in this epoch
    pub batch_count: u64,
    /// Total blocks in this epoch
    pub block_count: u64,
    /// Epoch duration in seconds
    pub duration_seconds: u64,
    /// Average batch size
    pub avg_batch_size_bytes: u64,
}
