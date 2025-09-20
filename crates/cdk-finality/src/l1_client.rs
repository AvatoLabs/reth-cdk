//! L1 client for finality checking using Alloy Provider

use crate::{FinalityError, FinalityResult, OracleMetadata};
use alloy_primitives::{Address, FixedBytes, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::BlockId;
use alloy_network::Ethereum;
use std::time::Duration;
use tracing::{debug, info};

/// L1 client configuration
#[derive(Debug, Clone)]
pub struct L1ClientConfig {
    /// L1 RPC URL
    pub rpc_url: String,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// API key for authentication (optional)
    pub api_key: Option<String>,
}

impl Default for L1ClientConfig {
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            api_key: None,
        }
    }
}

/// L1 client for interacting with Ethereum mainnet using Alloy Provider
pub struct L1Client {
    #[allow(dead_code)]
    config: L1ClientConfig,
    provider: Box<dyn Provider<Ethereum> + Send + Sync>,
    chain_id: Option<u64>,
}

impl L1Client {
    /// Create a new L1 client
    pub fn new(config: L1ClientConfig) -> FinalityResult<Self> {
        let provider = ProviderBuilder::new()
            .connect_http(config.rpc_url.parse().map_err(|e| {
                FinalityError::ConfigError(format!("Invalid RPC URL: {}", e))
            })?);

        Ok(Self {
            config,
            provider: Box::new(provider),
            chain_id: None,
        })
    }

    /// Create from RPC URL string
    pub fn from_rpc_url(rpc_url: &str) -> FinalityResult<Self> {
        let config = L1ClientConfig {
            rpc_url: rpc_url.to_string(),
            ..Default::default()
        };
        Self::new(config)
    }

    /// Initialize the client (get chain ID, etc.)
    pub async fn initialize(&mut self) -> FinalityResult<()> {
        debug!("Initializing L1 client");
        
        // Get chain ID using Alloy Provider
        self.chain_id = Some(self.get_chain_id().await?);

        info!("L1 client initialized for chain ID: {:?}", self.chain_id);
        Ok(())
    }

    /// Get chain ID using Alloy Provider
    async fn get_chain_id(&self) -> FinalityResult<u64> {
        let chain_id = self.provider.get_chain_id().await
            .map_err(|e| FinalityError::L1RpcError(format!("Failed to get chain ID: {}", e)))?;
        
        Ok(chain_id)
    }

    /// Get current L1 block number using Alloy Provider
    pub async fn get_current_block_number(&self) -> FinalityResult<u64> {
        let block_number = self.provider.get_block_number().await
            .map_err(|e| FinalityError::L1RpcError(format!("Failed to get block number: {}", e)))?;

        debug!("Current L1 block number: {}", block_number);
        Ok(block_number)
    }

    /// Get block by number using Alloy Provider
    pub async fn get_block_by_number(&self, block_number: u64) -> FinalityResult<Option<L1Block>> {
        let block_id = BlockId::Number(block_number.into());
        let block = self.provider.get_block(block_id).await
            .map_err(|e| FinalityError::L1RpcError(format!("Failed to get block: {}", e)))?;
        
        match block {
            Some(block) => {
                let l1_block = L1Block {
                    number: block_number,
                    hash: block.header.hash,
                    parent_hash: block.header.parent_hash,
                    timestamp: block.header.timestamp,
                    gas_limit: block.header.gas_limit,
                    gas_used: block.header.gas_used,
                    base_fee_per_gas: block.header.base_fee_per_gas,
                    transactions: vec![], // Simplified for now
                };
                Ok(Some(l1_block))
            }
            None => Ok(None),
        }
    }

    /// Call a contract method using Alloy Provider
    pub async fn call_contract(
        &self,
        address: Address,
        data: &[u8],
        _block_number: Option<u64>,
    ) -> FinalityResult<Vec<u8>> {
        // Simplified implementation - in practice, you would use proper contract calling
        // For now, we'll return an empty result as this is a placeholder
        debug!("Contract call to {:?} with {} bytes of data", address, data.len());
        Ok(vec![])
    }

    /// Health check
    pub async fn health_check(&self) -> FinalityResult<()> {
        debug!("Performing L1 client health check");
        
        // Try to get the latest block number
        self.get_current_block_number().await?;
        
        debug!("L1 client health check passed");
        Ok(())
    }

    /// Get client metadata
    pub async fn get_metadata(&self) -> FinalityResult<OracleMetadata> {
        let chain_id = self.chain_id.unwrap_or(1);
        let current_block = self.get_current_block_number().await?;
        
        let metadata = OracleMetadata::new(
            "L1 Client".to_string(),
            "1.0".to_string(),
            chain_id,
            Address::ZERO, // Bridge address will be set by oracle
        ).update_l1_block(current_block);

        Ok(metadata)
    }
}

/// L1 block data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1Block {
    pub number: u64,
    pub hash: FixedBytes<32>,
    pub parent_hash: FixedBytes<32>,
    pub timestamp: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub base_fee_per_gas: Option<u64>,
    pub transactions: Vec<L1Transaction>,
}

/// L1 transaction data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1Transaction {
    pub hash: FixedBytes<32>,
    pub from: Address,
    pub to: Option<Address>,
    pub value: U256,
    pub gas_price: Option<u64>,
    pub gas_limit: u64,
    pub nonce: u64,
    pub data: alloy_primitives::Bytes,
}

/// L1 transaction receipt
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1TransactionReceipt {
    pub transaction_hash: FixedBytes<32>,
    pub block_number: u64,
    pub block_hash: FixedBytes<32>,
    pub gas_used: u64,
    pub status: Option<u64>,
    pub logs: Vec<L1Log>,
}

/// L1 log data
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct L1Log {
    pub address: Address,
    pub topics: Vec<FixedBytes<32>>,
    pub data: alloy_primitives::Bytes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_l1_client_config_default() {
        let config = L1ClientConfig::default();
        assert_eq!(config.rpc_url, "http://localhost:8545");
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }

    #[tokio::test]
    async fn test_oracle_metadata_creation() {
        let metadata = OracleMetadata::new(
            "Test Oracle".to_string(),
            "1.0".to_string(),
            1,
            Address::ZERO,
        );

        assert_eq!(metadata.name, "Test Oracle");
        assert_eq!(metadata.version, "1.0");
        assert_eq!(metadata.l1_chain_id, 1);
        assert!(metadata.active);
    }

    #[tokio::test]
    async fn test_oracle_metadata_update() {
        let metadata = OracleMetadata::new(
            "Test Oracle".to_string(),
            "1.0".to_string(),
            1,
            Address::ZERO,
        ).update_l1_block(1000);

        assert_eq!(metadata.current_l1_block, 1000);
        assert!(metadata.last_check > 0);
    }
}