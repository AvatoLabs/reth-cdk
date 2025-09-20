//! Core traits for finality oracle

use cdk_types::{FinalityTag, FinalityStatus};
use crate::{FinalityResult};
use async_trait::async_trait;
use std::fmt::Debug;

/// A finality oracle that checks L1 finality status
#[async_trait]
pub trait FinalityOracle: Send + Sync + Debug {
    /// Poll for new finality updates
    async fn poll(&mut self) -> FinalityResult<Vec<FinalityTag>>;

    /// Get the current finality status for a specific batch
    async fn get_finality_status(&self, batch_id: u64) -> FinalityResult<Option<FinalityStatus>>;

    /// Get all finalized batches
    async fn get_finalized_batches(&self) -> FinalityResult<Vec<FinalityTag>>;

    /// Get all rolled back batches
    async fn get_rolled_back_batches(&self) -> FinalityResult<Vec<FinalityTag>>;

    /// Check if the oracle is healthy and ready to provide finality data
    async fn health_check(&self) -> FinalityResult<()>;

    /// Get oracle metadata
    async fn metadata(&self) -> FinalityResult<OracleMetadata>;

    /// Set the polling interval
    fn set_polling_interval(&mut self, interval: std::time::Duration);

    /// Get the current polling interval
    fn get_polling_interval(&self) -> std::time::Duration;
}

/// Oracle metadata
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct OracleMetadata {
    /// Oracle name
    pub name: String,
    /// Oracle version
    pub version: String,
    /// L1 chain ID
    pub l1_chain_id: u64,
    /// Bridge contract address
    pub bridge_address: alloy_primitives::Address,
    /// Current L1 block number
    pub current_l1_block: u64,
    /// Last finality check timestamp
    pub last_check: u64,
    /// Whether the oracle is currently active
    pub active: bool,
}

impl OracleMetadata {
    /// Create new oracle metadata
    pub fn new(
        name: String,
        version: String,
        l1_chain_id: u64,
        bridge_address: alloy_primitives::Address,
    ) -> Self {
        Self {
            name,
            version,
            l1_chain_id,
            bridge_address,
            current_l1_block: 0,
            last_check: 0,
            active: true,
        }
    }

    /// Update current L1 block
    pub fn update_l1_block(mut self, block_number: u64) -> Self {
        self.current_l1_block = block_number;
        self.last_check = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self
    }

    /// Set active status
    pub fn set_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// Finality update event
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalityUpdate {
    /// The finality tag
    pub tag: FinalityTag,
    /// Event type
    pub event_type: FinalityEventType,
    /// L1 block number when this event occurred
    pub l1_block_number: u64,
    /// Transaction hash that triggered this event
    pub tx_hash: Option<alloy_primitives::FixedBytes<32>>,
    /// Timestamp when this event was detected
    pub detected_at: u64,
}

/// Type of finality event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalityEventType {
    /// Batch was finalized
    Finalized,
    /// Batch was rolled back
    RolledBack,
    /// Finality status changed
    StatusChanged,
}

/// Finality oracle configuration
#[derive(Debug, Clone)]
pub struct FinalityOracleConfig {
    /// L1 RPC URL
    pub l1_rpc_url: String,
    /// Bridge contract address
    pub bridge_address: alloy_primitives::Address,
    /// Polling interval
    pub polling_interval: std::time::Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: std::time::Duration,
    /// Confirmation blocks required
    pub confirmation_blocks: u64,
    /// Enable strict mode
    pub strict_mode: bool,
}

impl Default for FinalityOracleConfig {
    fn default() -> Self {
        Self {
            l1_rpc_url: "http://localhost:8545".to_string(),
            bridge_address: alloy_primitives::Address::ZERO,
            polling_interval: std::time::Duration::from_secs(12), // ~1 L1 block
            max_retries: 3,
            retry_delay: std::time::Duration::from_secs(1),
            confirmation_blocks: 12, // ~2.5 minutes on Ethereum
            strict_mode: true,
        }
    }
}
