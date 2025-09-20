//! Integration tests for CDK observability

#[cfg(test)]
mod tests {
    use cdk_observe::{
        ObservabilityConfig, CdkMetrics, MetricsServer, TracingConfig, CdkTracing,
    };
    use alloy_primitives::U256;
    use std::net::SocketAddr;

    #[test]
    fn test_observability_config_creation() {
        let config = ObservabilityConfig::default();
        assert!(config.enable_logging);
        assert!(config.enable_metrics);
        assert!(config.enable_tracing);
        assert_eq!(config.log_level, "info");
    }

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

    #[test]
    fn test_tracing_config_creation() {
        let config = ObservabilityConfig::default();
        let _tracing_config = TracingConfig::new(&config);
        
        // Test that tracing config can be created without panicking
        // Note: level field is private, so we can't test it directly
    }

    #[test]
    fn test_tracing_spans() {
        // Initialize tracing for tests
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
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
            .with_max_level(tracing::Level::DEBUG)
            .try_init();

        let batch_id = U256::from(1);
        
        // Test logging functions (they should not panic)
        CdkTracing::log_ingestion_start(batch_id, 10);
        CdkTracing::log_ingestion_complete(batch_id, 100);
        CdkTracing::log_finality_update(batch_id, "finalized");
        CdkTracing::log_reorg_detected(U256::from(1), U256::from(2));
        CdkTracing::log_error("test_operation", "test_error");
    }

    #[test]
    fn test_config_serialization() {
        let config = ObservabilityConfig::default();
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ObservabilityConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(config.log_level, deserialized.log_level);
    }

    #[test]
    fn test_config_builder_pattern() {
        let config = ObservabilityConfig::new()
            .with_logging("debug", cdk_observe::LogFormat::Json)
            .with_metrics("127.0.0.1:9090".parse().unwrap())
            .with_tracing(true);

        assert_eq!(config.log_level, "debug");
        assert!(matches!(config.log_format, cdk_observe::LogFormat::Json));
        assert_eq!(config.metrics_address.to_string(), "127.0.0.1:9090");
        assert!(config.enable_tracing);
    }

    #[test]
    fn test_batch_metrics_config() {
        let config = cdk_observe::BatchMetricsConfig::default();
        assert!(config.enable_batch_height);
        assert!(config.enable_epoch_height);
        assert!(config.enable_ingest_tps);
        assert!(config.enable_processing_time);
    }

    #[test]
    fn test_finality_metrics_config() {
        let config = cdk_observe::FinalityMetricsConfig::default();
        assert!(config.enable_l1_lag);
        assert!(config.enable_reorg_count);
        assert!(config.enable_finality_status);
        assert!(config.enable_rollback_metrics);
    }

    #[test]
    fn test_log_format_enum() {
        let json_format = cdk_observe::LogFormat::Json;
        let pretty_format = cdk_observe::LogFormat::Pretty;
        let compact_format = cdk_observe::LogFormat::Compact;
        
        // Test that enum variants can be created without panicking
        assert!(matches!(json_format, cdk_observe::LogFormat::Json));
        assert!(matches!(pretty_format, cdk_observe::LogFormat::Pretty));
        assert!(matches!(compact_format, cdk_observe::LogFormat::Compact));
    }
}