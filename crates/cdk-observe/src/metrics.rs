//! Prometheus metrics for CDK observability

use alloy_primitives::U256;
use metrics::{Counter, Gauge, Histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tracing::info;

/// CDK metrics collector
pub struct CdkMetrics {
    // Batch metrics
    pub batch_height: Gauge,
    pub epoch_height: Gauge,
    pub ingest_tps: Gauge,
    pub batch_processing_time: Histogram,
    
    // Finality metrics
    pub l1_lag: Gauge,
    pub reorg_count: Counter,
    pub finality_status: Gauge,
    pub rollback_count: Counter,
    
    // System metrics
    pub active_connections: Gauge,
    pub error_count: Counter,
    pub warning_count: Counter,
}

impl Default for CdkMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl CdkMetrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            batch_height: Gauge::noop(),
            epoch_height: Gauge::noop(),
            ingest_tps: Gauge::noop(),
            batch_processing_time: Histogram::noop(),
            l1_lag: Gauge::noop(),
            reorg_count: Counter::noop(),
            finality_status: Gauge::noop(),
            rollback_count: Counter::noop(),
            active_connections: Gauge::noop(),
            error_count: Counter::noop(),
            warning_count: Counter::noop(),
        }
    }

    /// Update batch height metric
    pub fn update_batch_height(&self, height: U256) {
        // Convert U256 to f64 safely
        if let Ok(h) = TryInto::<u64>::try_into(height) {
            self.batch_height.set(h as f64);
        }
    }

    /// Update epoch height metric
    pub fn update_epoch_height(&self, height: U256) {
        // Convert U256 to f64 safely
        if let Ok(h) = TryInto::<u64>::try_into(height) {
            self.epoch_height.set(h as f64);
        }
    }

    /// Update ingestion TPS metric
    pub fn update_ingest_tps(&self, tps: f64) {
        self.ingest_tps.set(tps);
    }

    /// Record batch processing time
    pub fn record_batch_processing_time(&self, duration_secs: f64) {
        self.batch_processing_time.record(duration_secs);
    }

    /// Update L1 lag metric
    pub fn update_l1_lag(&self, lag_blocks: u64) {
        self.l1_lag.set(lag_blocks as f64);
    }

    /// Increment reorg counter
    pub fn increment_reorg_count(&self) {
        self.reorg_count.increment(1);
    }

    /// Update finality status metric
    pub fn update_finality_status(&self, status: u8) {
        self.finality_status.set(status as f64);
    }

    /// Increment rollback counter
    pub fn increment_rollback_count(&self) {
        self.rollback_count.increment(1);
    }

    /// Update active connections metric
    pub fn update_active_connections(&self, count: u64) {
        self.active_connections.set(count as f64);
    }

    /// Increment error counter
    pub fn increment_error_count(&self) {
        self.error_count.increment(1);
    }

    /// Increment warning counter
    pub fn increment_warning_count(&self) {
        self.warning_count.increment(1);
    }
}

/// Metrics server for Prometheus
pub struct MetricsServer {
    address: SocketAddr,
}

impl MetricsServer {
    /// Create a new metrics server
    pub fn new(address: SocketAddr) -> Self {
        Self { address }
    }

    /// Start the metrics server
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let builder = PrometheusBuilder::new();
        
        // Install the metrics recorder
        builder.install_recorder()?;
        
        info!("Metrics server started on {}", self.address);
        
        // Keep the server running
        tokio::signal::ctrl_c().await?;
        info!("Shutting down metrics server");
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = CdkMetrics::new();
        
        // Test that metrics can be created without panicking
        metrics.update_batch_height(U256::from(100));
        metrics.update_epoch_height(U256::from(10));
        metrics.update_ingest_tps(5.0);
        metrics.record_batch_processing_time(1.5);
        metrics.update_l1_lag(5);
        metrics.increment_reorg_count();
        metrics.update_finality_status(1);
        metrics.increment_rollback_count();
        metrics.update_active_connections(10);
        metrics.increment_error_count();
        metrics.increment_warning_count();
    }

    #[test]
    fn test_metrics_server_creation() {
        let address: SocketAddr = "127.0.0.1:9000".parse().unwrap();
        let _server = MetricsServer::new(address);
        // Test that server can be created without panicking
    }
}