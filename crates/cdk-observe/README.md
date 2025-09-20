# CDK Observe

CDK Observability and Monitoring crate for unified logging, metrics, and tracing support.

## Features

- **Unified Logging**: Structured logging with configurable formats (JSON, Pretty, Compact)
- **Prometheus Metrics**: Comprehensive metrics collection for CDK operations
- **Tracing Support**: Distributed tracing with spans for batch, epoch, and finality operations
- **Configurable**: Flexible configuration for different environments

## Metrics

### Batch Metrics
- `cdk_batch_height`: Current batch height
- `cdk_epoch_height`: Current epoch height  
- `cdk_ingest_tps`: Batches ingested per second
- `cdk_batch_processing_time_seconds`: Time to process a batch

### Finality Metrics
- `cdk_l1_lag_blocks`: Number of blocks behind L1
- `cdk_reorg_total`: Total number of reorganizations
- `cdk_finality_status`: Finality status (0=pending, 1=finalized)
- `cdk_rollback_total`: Total number of rollbacks

### System Metrics
- `cdk_active_connections`: Number of active connections
- `cdk_errors_total`: Total number of errors
- `cdk_warnings_total`: Total number of warnings

## Usage

### Basic Setup

```rust
use cdk_observe::{ObservabilityConfig, LogFormat, MetricsManager, init_with_config};

// Configure observability
let config = ObservabilityConfig::new()
    .with_logging("info", LogFormat::Pretty)
    .with_metrics("127.0.0.1:9000".parse().unwrap())
    .with_tracing(true);

// Initialize logging
init_with_config(config.clone())?;

// Setup metrics
let metrics_manager = MetricsManager::new()
    .with_server(config.metrics_address);
let metrics = metrics_manager.install_recorder()?;

// Start metrics server
tokio::spawn(async move {
    metrics_manager.start_server().await.unwrap();
});
```

### Using Metrics

```rust
use cdk_observe::{CdkMetrics, FinalityStatus};
use alloy_primitives::U256;

let metrics = CdkMetrics::new();

// Update batch metrics
metrics.update_batch_height(U256::from(100));
metrics.update_epoch_height(U256::from(10));
metrics.update_ingest_tps(5.5);

// Update finality metrics
metrics.update_l1_lag(12);
metrics.update_finality_status(FinalityStatus::Finalized);

// Record processing time
metrics.record_batch_processing_time(1.5);

// Increment counters
metrics.increment_reorg_count();
metrics.increment_error_count();
```

### Using Tracing

```rust
use cdk_observe::utils::{batch_span, with_batch_span};
use alloy_primitives::U256;

// Create spans manually
let batch_id = U256::from(1);
let batch_height = U256::from(100);
let span = batch_span(batch_id, batch_height);
let _guard = span.enter();

// Or use the convenience function
let result = with_batch_span(batch_id, batch_height, || async {
    // Your batch processing logic here
    "processed"
}).await;
```

## Configuration

### Log Formats

- **JSON**: Structured logging for production environments
- **Pretty**: Human-readable format for development
- **Compact**: Minimal format for high-throughput scenarios

### Metrics Configuration

```rust
use cdk_observe::{BatchMetricsConfig, FinalityMetricsConfig};

let batch_config = BatchMetricsConfig {
    enable_batch_height: true,
    enable_epoch_height: true,
    enable_ingest_tps: true,
    enable_processing_time: true,
};

let finality_config = FinalityMetricsConfig {
    enable_l1_lag: true,
    enable_reorg_count: true,
    enable_finality_status: true,
    enable_rollback_metrics: true,
};

let config = ObservabilityConfig::new()
    .with_batch_metrics(batch_config)
    .with_finality_metrics(finality_config);
```

## Integration

This crate integrates with other CDK modules:

- **cdk-datastream**: Metrics for data source health and performance
- **cdk-ingest**: Metrics for batch processing and assembly
- **cdk-finality**: Metrics for L1 finality and rollback operations
- **cdk-rpc-ext**: Metrics for RPC API usage and performance

## Examples

See the `examples/` directory for complete usage examples.

## Testing

Run the tests with:

```bash
cargo test
```

Run integration tests with:

```bash
cargo test --test integration
```

## License

MIT OR Apache-2.0
