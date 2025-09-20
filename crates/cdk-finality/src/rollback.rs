//! Rollback management for finality operations

use crate::{FinalityError, FinalityResult, FinalityUpdate, FinalityEventType};
use alloy_primitives::FixedBytes;
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Rollback manager for handling batch rollbacks
#[derive(Debug)]
pub struct RollbackManager {
    /// Rollback history
    rollback_history: HashMap<u64, RollbackRecord>,
    /// Pending rollbacks
    pending_rollbacks: HashMap<u64, PendingRollback>,
    /// Rollback configuration
    config: RollbackConfig,
}

/// Rollback record
#[derive(Debug, Clone, PartialEq)]
pub struct RollbackRecord {
    /// Batch ID that was rolled back
    pub batch_id: u64,
    /// Batch hash
    pub batch_hash: FixedBytes<32>,
    /// L1 block number when rollback occurred
    pub l1_block_number: u64,
    /// Transaction hash that triggered rollback
    pub tx_hash: Option<FixedBytes<32>>,
    /// Timestamp when rollback was detected
    pub timestamp: u64,
    /// Reason for rollback
    pub reason: String,
    /// Blocks affected by this rollback
    pub affected_blocks: Vec<u64>,
}

/// Pending rollback
#[derive(Debug, Clone, PartialEq)]
pub struct PendingRollback {
    /// Batch ID
    pub batch_id: u64,
    /// Batch hash
    pub batch_hash: FixedBytes<32>,
    /// L1 block number
    pub l1_block_number: u64,
    /// Transaction hash
    pub tx_hash: Option<FixedBytes<32>>,
    /// Timestamp when rollback was detected
    pub timestamp: u64,
    /// Confirmation count
    pub confirmations: u64,
    /// Required confirmations
    pub required_confirmations: u64,
}

/// Rollback configuration
#[derive(Debug, Clone)]
pub struct RollbackConfig {
    /// Required confirmations before executing rollback
    pub required_confirmations: u64,
    /// Maximum rollback depth
    pub max_rollback_depth: u64,
    /// Rollback timeout
    pub rollback_timeout: std::time::Duration,
    /// Enable automatic rollback execution
    pub auto_execute: bool,
    /// Enable rollback validation
    pub validate_rollbacks: bool,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            required_confirmations: 12, // ~2.5 minutes on Ethereum
            max_rollback_depth: 1000,   // Maximum 1000 blocks
            rollback_timeout: std::time::Duration::from_secs(3600), // 1 hour
            auto_execute: true,
            validate_rollbacks: true,
        }
    }
}

impl RollbackManager {
    /// Create a new rollback manager
    pub fn new(config: RollbackConfig) -> Self {
        Self {
            rollback_history: HashMap::new(),
            pending_rollbacks: HashMap::new(),
            config,
        }
    }

    /// Process a finality update
    pub async fn process_finality_update(
        &mut self,
        update: FinalityUpdate,
    ) -> FinalityResult<Vec<RollbackAction>> {
        debug!("Processing finality update: {:?}", update);

        match update.event_type {
            FinalityEventType::RolledBack => {
                self.handle_rollback(update).await
            }
            FinalityEventType::Finalized => {
                self.handle_finalization(update).await
            }
            FinalityEventType::StatusChanged => {
                self.handle_status_change(update).await
            }
        }
    }

    /// Handle rollback event
    async fn handle_rollback(
        &mut self,
        update: FinalityUpdate,
    ) -> FinalityResult<Vec<RollbackAction>> {
        let batch_id = update.tag.batch_id.to::<u64>();
        
        // Check if rollback is already processed
        if self.rollback_history.contains_key(&batch_id) {
            warn!("Rollback for batch {} already processed", batch_id);
            return Ok(vec![]);
        }

        // Create pending rollback
        let pending_rollback = PendingRollback {
            batch_id,
            batch_hash: update.tag.l1_block_hash,
            l1_block_number: update.l1_block_number,
            tx_hash: update.tx_hash,
            timestamp: update.detected_at,
            confirmations: 0,
            required_confirmations: self.config.required_confirmations,
        };

        self.pending_rollbacks.insert(batch_id, pending_rollback);

        if self.config.auto_execute {
            // Check if we have enough confirmations
            if self.check_rollback_confirmations(batch_id).await? {
                return self.execute_rollback(batch_id).await;
            }
        }

        Ok(vec![RollbackAction::PendingRollback(batch_id)])
    }

    /// Handle finalization event
    async fn handle_finalization(
        &mut self,
        update: FinalityUpdate,
    ) -> FinalityResult<Vec<RollbackAction>> {
        let batch_id = update.tag.batch_id.to::<u64>();
        
        // Remove from pending rollbacks if it was there
        if self.pending_rollbacks.remove(&batch_id).is_some() {
            info!("Batch {} was finalized, removing from pending rollbacks", batch_id);
        }

        Ok(vec![RollbackAction::Finalized(batch_id)])
    }

    /// Handle status change event
    async fn handle_status_change(
        &mut self,
        update: FinalityUpdate,
    ) -> FinalityResult<Vec<RollbackAction>> {
        debug!("Status change for batch {}: {:?}", update.tag.batch_id, update.tag.status);
        Ok(vec![RollbackAction::StatusChanged(update.tag.batch_id.to::<u64>())])
    }

    /// Check rollback confirmations
    async fn check_rollback_confirmations(&mut self, batch_id: u64) -> FinalityResult<bool> {
        if let Some(pending) = self.pending_rollbacks.get_mut(&batch_id) {
            pending.confirmations += 1;
            
            if pending.confirmations >= pending.required_confirmations {
                debug!("Rollback for batch {} has enough confirmations", batch_id);
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Execute rollback
    async fn execute_rollback(&mut self, batch_id: u64) -> FinalityResult<Vec<RollbackAction>> {
        let pending = self.pending_rollbacks.remove(&batch_id)
            .ok_or_else(|| FinalityError::RollbackError(format!("No pending rollback for batch {}", batch_id)))?;

        // Create rollback record
        let rollback_record = RollbackRecord {
            batch_id,
            batch_hash: pending.batch_hash,
            l1_block_number: pending.l1_block_number,
            tx_hash: pending.tx_hash,
            timestamp: pending.timestamp,
            reason: "L1 finality rollback".to_string(),
            affected_blocks: self.calculate_affected_blocks(batch_id).await?,
        };

        self.rollback_history.insert(batch_id, rollback_record);

        info!("Executing rollback for batch {} affecting {} blocks", 
              batch_id, self.rollback_history[&batch_id].affected_blocks.len());

        Ok(vec![RollbackAction::ExecuteRollback(batch_id)])
    }

    /// Calculate affected blocks for a rollback
    async fn calculate_affected_blocks(&self, batch_id: u64) -> FinalityResult<Vec<u64>> {
        // In a real implementation, this would query the database
        // to find all blocks that belong to this batch
        // For now, we return a placeholder
        Ok(vec![batch_id * 100, batch_id * 100 + 1, batch_id * 100 + 2])
    }

    /// Get rollback history
    pub fn get_rollback_history(&self) -> &HashMap<u64, RollbackRecord> {
        &self.rollback_history
    }

    /// Get pending rollbacks
    pub fn get_pending_rollbacks(&self) -> &HashMap<u64, PendingRollback> {
        &self.pending_rollbacks
    }

    /// Check if a batch was rolled back
    pub fn is_batch_rolled_back(&self, batch_id: u64) -> bool {
        self.rollback_history.contains_key(&batch_id)
    }

    /// Get rollback record for a batch
    pub fn get_rollback_record(&self, batch_id: u64) -> Option<&RollbackRecord> {
        self.rollback_history.get(&batch_id)
    }

    /// Clean up old rollback records
    pub fn cleanup_old_records(&mut self, max_age: std::time::Duration) {
        let cutoff_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - max_age.as_secs();

        self.rollback_history.retain(|_, record| record.timestamp > cutoff_time);
        self.pending_rollbacks.retain(|_, pending| pending.timestamp > cutoff_time);
    }
}

/// Rollback action
#[derive(Debug, Clone, PartialEq)]
pub enum RollbackAction {
    /// Execute rollback for batch
    ExecuteRollback(u64),
    /// Rollback is pending confirmation
    PendingRollback(u64),
    /// Batch was finalized
    Finalized(u64),
    /// Status changed
    StatusChanged(u64),
}

/// Rollback statistics
#[derive(Debug, Clone, PartialEq)]
pub struct RollbackStats {
    /// Total rollbacks executed
    pub total_rollbacks: u64,
    /// Pending rollbacks
    pub pending_rollbacks: u64,
    /// Average rollback depth
    pub avg_rollback_depth: f64,
    /// Last rollback timestamp
    pub last_rollback: u64,
    /// Rollback success rate
    pub success_rate: f64,
}

impl Default for RollbackStats {
    fn default() -> Self {
        Self {
            total_rollbacks: 0,
            pending_rollbacks: 0,
            avg_rollback_depth: 0.0,
            last_rollback: 0,
            success_rate: 100.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[tokio::test]
    async fn test_rollback_manager_creation() {
        let config = RollbackConfig::default();
        let manager = RollbackManager::new(config);
        
        assert_eq!(manager.get_rollback_history().len(), 0);
        assert_eq!(manager.get_pending_rollbacks().len(), 0);
    }

    #[tokio::test]
    async fn test_rollback_config_default() {
        let config = RollbackConfig::default();
        assert_eq!(config.required_confirmations, 12);
        assert_eq!(config.max_rollback_depth, 1000);
        assert!(config.auto_execute);
        assert!(config.validate_rollbacks);
    }

    #[tokio::test]
    async fn test_rollback_record_creation() {
        let record = RollbackRecord {
            batch_id: 1,
            batch_hash: FixedBytes::from([1u8; 32]),
            l1_block_number: 1000,
            tx_hash: Some(FixedBytes::from([2u8; 32])),
            timestamp: 1234567890,
            reason: "Test rollback".to_string(),
            affected_blocks: vec![100, 101, 102],
        };

        assert_eq!(record.batch_id, 1);
        assert_eq!(record.affected_blocks.len(), 3);
    }

    #[tokio::test]
    async fn test_rollback_stats_default() {
        let stats = RollbackStats::default();
        assert_eq!(stats.total_rollbacks, 0);
        assert_eq!(stats.pending_rollbacks, 0);
        assert_eq!(stats.success_rate, 100.0);
    }
}
