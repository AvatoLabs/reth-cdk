//! CDK RPC API implementation

use async_trait::async_trait;
use alloy_primitives::U256;
use tracing::{info, warn, instrument};

use crate::{
    CdkRpcError, CdkRpcResult,
    types::*,
};
use cdk_types::{Batch, BatchId, Epoch};
use cdk_datastream::BatchSource;
use cdk_ingest::MappingStorage;
use cdk_finality::FinalityOracle;

/// CDK RPC API trait definition
#[async_trait]
pub trait CdkRpcApi {
    /// Get batch information by batch number
    async fn get_batch_by_number(&self, batch_number: String) -> Result<Option<BatchResponse>, CdkRpcError>;

    /// Get epoch information by block number
    async fn get_epoch_by_block(&self, block_number: String) -> Result<Option<EpochResponse>, CdkRpcError>;

    /// Get the latest finalized batch
    async fn finalized_batch(&mut self) -> Result<Option<FinalizedBatchResponse>, CdkRpcError>;

    /// Get CDK metrics and statistics
    async fn metrics(&self) -> Result<CdkMetrics, CdkRpcError>;
}

/// CDK RPC API implementation
pub struct CdkRpcApiImpl {
    batch_source: Box<dyn BatchSource + Send + Sync>,
    mapping_storage: Box<dyn MappingStorage + Send + Sync>,
    finality_oracle: Box<dyn FinalityOracle + Send + Sync>,
}

impl CdkRpcApiImpl {
    /// Create a new CDK RPC API implementation
    pub fn new(
        batch_source: Box<dyn BatchSource + Send + Sync>,
        mapping_storage: Box<dyn MappingStorage + Send + Sync>,
        finality_oracle: Box<dyn FinalityOracle + Send + Sync>,
    ) -> Self {
        Self {
            batch_source,
            mapping_storage,
            finality_oracle,
        }
    }

    /// Parse hex string to U256
    fn parse_hex_number(hex_str: &str) -> CdkRpcResult<U256> {
        let cleaned = hex_str.strip_prefix("0x").unwrap_or(hex_str);
        U256::from_str_radix(cleaned, 16)
            .map_err(|_| CdkRpcError::InvalidParameter(format!("Invalid hex number: {}", hex_str)))
    }

    /// Get batch metadata
    async fn get_batch_metadata(&self, batch: &Batch) -> CdkRpcResult<BatchMetadata> {
        let block_count = batch.blocks.len() as u64;
        // Note: BlockInBatch doesn't have transactions field, using placeholder
        let transaction_count = 0u64;
        
        // Estimate size (simplified)
        let size_bytes = serde_json::to_vec(batch)
            .map_err(|e| CdkRpcError::InternalError(e.to_string()))?
            .len() as u64;

        Ok(BatchMetadata {
            block_count,
            transaction_count,
            size_bytes,
            processing_time_ms: 0, // TODO: Track actual processing time
        })
    }

    /// Get epoch metadata
    async fn get_epoch_metadata(&self, epoch: &Epoch) -> CdkRpcResult<EpochMetadata> {
        let batch_count = (epoch.end_batch - epoch.start_batch).to::<u64>() + 1;
        let block_count = (epoch.end_block - epoch.start_block).to::<u64>() + 1;
        let duration_seconds = epoch.end_timestamp - epoch.start_timestamp;

        Ok(EpochMetadata {
            batch_count,
            block_count,
            duration_seconds,
            avg_batch_size_bytes: 0, // TODO: Calculate actual average
        })
    }
}

#[async_trait]
impl CdkRpcApi for CdkRpcApiImpl {
    #[instrument(skip(self), fields(batch_number = %batch_number))]
    async fn get_batch_by_number(&self, batch_number: String) -> Result<Option<BatchResponse>, CdkRpcError> {
        info!("Getting batch by number: {}", batch_number);
        
        let _batch_num = Self::parse_hex_number(&batch_number)?;
        
        // TODO: Implement actual batch retrieval from storage
        // For now, return None as placeholder
        warn!("Batch retrieval not yet implemented");
        Ok(None)
    }

    #[instrument(skip(self), fields(block_number = %block_number))]
    async fn get_epoch_by_block(&self, block_number: String) -> Result<Option<EpochResponse>, CdkRpcError> {
        info!("Getting epoch by block number: {}", block_number);
        
        let _block_num = Self::parse_hex_number(&block_number)?;
        
        // TODO: Implement actual epoch retrieval from storage
        // For now, return None as placeholder
        warn!("Epoch retrieval not yet implemented");
        Ok(None)
    }

    #[instrument(skip(self))]
    async fn finalized_batch(&mut self) -> Result<Option<FinalizedBatchResponse>, CdkRpcError> {
        info!("Getting finalized batch");
        
        // Poll finality oracle for latest finality tags
        let finality_tags = self.finality_oracle.poll().await
            .map_err(|e| CdkRpcError::FinalityOracleError(e.to_string()))?;
        
        if let Some(latest_tag) = finality_tags.last() {
            Ok(Some(FinalizedBatchResponse {
                batch_id: BatchId::new(
                    latest_tag.batch_id,
                    latest_tag.l1_block_hash,
                ),
                status: format!("{:?}", latest_tag.status),
                l1_block: latest_tag.l1_block,
                timestamp: latest_tag.timestamp,
            }))
        } else {
            Ok(None)
        }
    }

    #[instrument(skip(self))]
    async fn metrics(&self) -> Result<CdkMetrics, CdkRpcError> {
        info!("Getting CDK metrics");
        
        // TODO: Implement actual metrics collection
        // For now, return placeholder metrics
        Ok(CdkMetrics {
            total_batches: 0,
            total_epochs: 0,
            latest_batch: None,
            latest_epoch: None,
            latest_finalized_batch: None,
            l1_lag: None,
            reorg_count: 0,
            ingest_tps: 0.0,
        })
    }
}