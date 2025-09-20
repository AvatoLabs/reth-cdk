//! Unified tracing configuration for CDK observability

use alloy_primitives::U256;
use tracing::{info, warn, Level};
use tracing_subscriber::EnvFilter;

/// Tracing configuration
pub struct TracingConfig {
    level: Level,
}

impl TracingConfig {
    /// Create a new tracing configuration
    pub fn new(config: &ObservabilityConfig) -> Self {
        let level = match config.log_level.as_str() {
            "trace" => Level::TRACE,
            "debug" => Level::DEBUG,
            "info" => Level::INFO,
            "warn" => Level::WARN,
            "error" => Level::ERROR,
            _ => Level::INFO,
        };

        Self { level }
    }

    /// Initialize tracing with the configuration
    pub fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let filter = EnvFilter::from_default_env()
            .add_directive(format!("{}", self.level).parse()?);

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .init();

        Ok(())
    }
}

/// Tracing utilities for CDK operations
pub struct CdkTracing;

impl CdkTracing {
    /// Create a span for batch processing
    pub async fn trace_batch_processing<F, Fut>(
        batch_id: U256,
        batch_height: U256,
        f: F,
    ) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let _span = tracing::info_span!("batch_processing", batch_id = %batch_id, batch_height = %batch_height);
        f().await
    }

    /// Create a span for epoch processing
    pub async fn trace_epoch_processing<F, Fut>(
        epoch_id: U256,
        epoch_height: U256,
        f: F,
    ) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let _span = tracing::info_span!("epoch_processing", epoch_id = %epoch_id, epoch_height = %epoch_height);
        f().await
    }

    /// Create a span for finality checking
    pub async fn trace_finality_check<F, Fut>(
        batch_id: U256,
        l1_block: U256,
        f: F,
    ) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let _span = tracing::info_span!("finality_check", batch_id = %batch_id, l1_block = %l1_block);
        f().await
    }

    /// Log ingestion start
    pub fn log_ingestion_start(batch_id: U256, block_count: usize) {
        info!("Starting ingestion for batch {} with {} blocks", batch_id, block_count);
    }

    /// Log ingestion completion
    pub fn log_ingestion_complete(batch_id: U256, duration_ms: u64) {
        info!("Completed ingestion for batch {} in {}ms", batch_id, duration_ms);
    }

    /// Log finality update
    pub fn log_finality_update(batch_id: U256, status: &str) {
        info!("Finality update for batch {}: {}", batch_id, status);
    }

    /// Log reorg detection
    pub fn log_reorg_detected(old_batch: U256, new_batch: U256) {
        warn!("Reorg detected: batch {} replaced by {}", old_batch, new_batch);
    }

    /// Log error
    pub fn log_error(operation: &str, error: &str) {
        tracing::error!("Error in {}: {}", operation, error);
    }
}

// Import the config types
use crate::config::ObservabilityConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracing_config_creation() {
        let config = ObservabilityConfig::default();
        let tracing_config = TracingConfig::new(&config);
        
        assert_eq!(tracing_config.level, Level::INFO);
    }

    #[test]
    fn test_tracing_spans() {
        // Initialize tracing for tests
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .try_init();

        let batch_id = U256::from(1);
        let batch_height = U256::from(100);
        let epoch_id = U256::from(1);
        let epoch_height = U256::from(10);
        let l1_block = U256::from(1000);

        // Test batch processing span
        let result = futures::executor::block_on(CdkTracing::trace_batch_processing(batch_id, batch_height, || async {
            "batch_result"
        }));
        assert_eq!(result, "batch_result");

        // Test epoch processing span
        let result = futures::executor::block_on(CdkTracing::trace_epoch_processing(epoch_id, epoch_height, || async {
            "epoch_result"
        }));
        assert_eq!(result, "epoch_result");

        // Test finality check span
        let result = futures::executor::block_on(CdkTracing::trace_finality_check(batch_id, l1_block, || async {
            "finality_result"
        }));
        assert_eq!(result, "finality_result");
    }

    #[test]
    fn test_logging_functions() {
        // Initialize tracing for tests
        let _ = tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .try_init();

        let batch_id = U256::from(1);
        
        // Test logging functions (they should not panic)
        CdkTracing::log_ingestion_start(batch_id, 10);
        CdkTracing::log_ingestion_complete(batch_id, 100);
        CdkTracing::log_finality_update(batch_id, "finalized");
        CdkTracing::log_reorg_detected(U256::from(1), U256::from(2));
        CdkTracing::log_error("test_operation", "test_error");
    }
}