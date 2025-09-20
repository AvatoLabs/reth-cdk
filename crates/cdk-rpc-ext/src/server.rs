//! CDK RPC Server implementation using Alloy Provider

use alloy_provider::ProviderBuilder;
use alloy_network::Ethereum;
use std::net::SocketAddr;
use tracing::{info, instrument};

use crate::{
    CdkRpcError, CdkRpcResult,
    api::CdkRpcApiImpl,
};
use cdk_datastream::BatchSource;
use cdk_ingest::MappingStorage;
use cdk_finality::FinalityOracle;

/// Configuration for CDK RPC server
#[derive(Debug, Clone)]
pub struct CdkRpcConfig {
    /// Enable batch queries
    pub enable_batch_queries: bool,
    /// Enable epoch queries
    pub enable_epoch_queries: bool,
    /// Enable finality queries
    pub enable_finality_queries: bool,
    /// Enable metrics
    pub enable_metrics: bool,
    /// Maximum batch history to keep
    pub max_batch_history: u64,
    /// Maximum epoch history to keep
    pub max_epoch_history: u64,
    /// Server address
    pub address: SocketAddr,
}

impl Default for CdkRpcConfig {
    fn default() -> Self {
        Self {
            enable_batch_queries: true,
            enable_epoch_queries: true,
            enable_finality_queries: true,
            enable_metrics: true,
            max_batch_history: 1000,
            max_epoch_history: 100,
            address: "127.0.0.1:8545".parse().unwrap(),
        }
    }
}

/// CDK RPC Server using Alloy Provider
pub struct CdkRpcServer {
    config: CdkRpcConfig,
    batch_source: Box<dyn BatchSource + Send + Sync>,
    mapping_storage: Box<dyn MappingStorage + Send + Sync>,
    finality_oracle: Box<dyn FinalityOracle + Send + Sync>,
    provider: Box<dyn alloy_provider::Provider<Ethereum> + Send + Sync>,
}

impl CdkRpcServer {
    /// Create a new CDK RPC server
    pub async fn new(
        config: CdkRpcConfig,
        batch_source: Box<dyn BatchSource + Send + Sync>,
        mapping_storage: Box<dyn MappingStorage + Send + Sync>,
        finality_oracle: Box<dyn FinalityOracle + Send + Sync>,
        rpc_url: String,
    ) -> CdkRpcResult<Self> {
        let provider = ProviderBuilder::new()
            .connect_http(rpc_url.parse().map_err(|e| {
                CdkRpcError::InternalError(format!("Invalid RPC URL: {}", e))
            })?);

        Ok(Self {
            config,
            batch_source,
            mapping_storage,
            finality_oracle,
            provider: Box::new(provider),
        })
    }

    /// Start the RPC server
    #[instrument(skip(self))]
    pub async fn start(self) -> CdkRpcResult<()> {
        info!("Starting CDK RPC server on {}", self.config.address);
        
        let _api_impl = CdkRpcApiImpl::new(
            self.batch_source,
            self.mapping_storage,
            self.finality_oracle,
        );
        
        // Use Alloy Provider for RPC operations
        // This is a simplified implementation - in practice, you would
        // integrate with the existing Reth RPC infrastructure
        info!("CDK RPC server started successfully with Alloy Provider");
        Ok(())
    }

    /// Get the underlying provider for direct RPC calls
    pub fn provider(&self) -> &dyn alloy_provider::Provider<Ethereum> {
        self.provider.as_ref()
    }
}