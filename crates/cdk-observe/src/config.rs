//! Configuration for CDK observability

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Configuration for CDK observability features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    /// Enable structured logging
    pub enable_logging: bool,
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
    /// Log format (json, pretty, compact)
    pub log_format: LogFormat,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Prometheus metrics endpoint address
    pub metrics_address: SocketAddr,
    /// Enable tracing spans
    pub enable_tracing: bool,
    /// Batch processing metrics
    pub batch_metrics: BatchMetricsConfig,
    /// Finality metrics
    pub finality_metrics: FinalityMetricsConfig,
}

/// Log format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    /// JSON format for structured logging
    Json,
    /// Pretty format for development
    Pretty,
    /// Compact format for production
    Compact,
}

/// Batch processing metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMetricsConfig {
    /// Enable batch height metrics
    pub enable_batch_height: bool,
    /// Enable epoch height metrics
    pub enable_epoch_height: bool,
    /// Enable ingestion TPS metrics
    pub enable_ingest_tps: bool,
    /// Enable batch processing time metrics
    pub enable_processing_time: bool,
}

/// Finality metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalityMetricsConfig {
    /// Enable L1 lag metrics
    pub enable_l1_lag: bool,
    /// Enable reorg count metrics
    pub enable_reorg_count: bool,
    /// Enable finality status metrics
    pub enable_finality_status: bool,
    /// Enable rollback metrics
    pub enable_rollback_metrics: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: "info".to_string(),
            log_format: LogFormat::Pretty,
            enable_metrics: true,
            metrics_address: "127.0.0.1:9000".parse().unwrap(),
            enable_tracing: true,
            batch_metrics: BatchMetricsConfig::default(),
            finality_metrics: FinalityMetricsConfig::default(),
        }
    }
}

impl Default for BatchMetricsConfig {
    fn default() -> Self {
        Self {
            enable_batch_height: true,
            enable_epoch_height: true,
            enable_ingest_tps: true,
            enable_processing_time: true,
        }
    }
}

impl Default for FinalityMetricsConfig {
    fn default() -> Self {
        Self {
            enable_l1_lag: true,
            enable_reorg_count: true,
            enable_finality_status: true,
            enable_rollback_metrics: true,
        }
    }
}

impl ObservabilityConfig {
    /// Create a new observability configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable logging with specific level and format
    pub fn with_logging(mut self, level: &str, format: LogFormat) -> Self {
        self.enable_logging = true;
        self.log_level = level.to_string();
        self.log_format = format;
        self
    }

    /// Enable metrics with specific address
    pub fn with_metrics(mut self, address: SocketAddr) -> Self {
        self.enable_metrics = true;
        self.metrics_address = address;
        self
    }

    /// Enable tracing
    pub fn with_tracing(mut self, enable: bool) -> Self {
        self.enable_tracing = enable;
        self
    }

    /// Configure batch metrics
    pub fn with_batch_metrics(mut self, config: BatchMetricsConfig) -> Self {
        self.batch_metrics = config;
        self
    }

    /// Configure finality metrics
    pub fn with_finality_metrics(mut self, config: FinalityMetricsConfig) -> Self {
        self.finality_metrics = config;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ObservabilityConfig::default();
        assert!(config.enable_logging);
        assert!(config.enable_metrics);
        assert!(config.enable_tracing);
        assert_eq!(config.log_level, "info");
        assert!(matches!(config.log_format, LogFormat::Pretty));
    }

    #[test]
    fn test_config_builder() {
        let config = ObservabilityConfig::new()
            .with_logging("debug", LogFormat::Json)
            .with_metrics("127.0.0.1:9090".parse().unwrap())
            .with_tracing(true);

        assert_eq!(config.log_level, "debug");
        assert!(matches!(config.log_format, LogFormat::Json));
        assert_eq!(config.metrics_address.to_string(), "127.0.0.1:9090");
        assert!(config.enable_tracing);
    }

    #[test]
    fn test_serialization() {
        let config = ObservabilityConfig::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ObservabilityConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.log_level, deserialized.log_level);
    }
}
