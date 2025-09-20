//! Performance monitoring and caching for CDK

use crate::{ObservabilityError, ObservabilityResult};
use alloy_primitives::U256;
use cdk_types::{Batch, Epoch, FinalityTag};
use moka::future::Cache;
use prometheus::{Counter, Histogram, Gauge, Registry, Opts, HistogramOpts};
use rayon::prelude::*;
use std::time::{Duration, Instant};
use tracing::{debug, info};

/// Performance metrics for CDK operations
pub struct PerformanceMetrics {
    /// Batch import counter
    pub batches_imported: Counter,
    /// Batch import duration histogram
    pub batch_import_duration: Histogram,
    /// Epoch processing counter
    pub epochs_processed: Counter,
    /// Epoch processing duration histogram
    pub epoch_processing_duration: Histogram,
    /// Finality check counter
    pub finality_checks: Counter,
    /// Finality check duration histogram
    pub finality_check_duration: Histogram,
    /// Current head block gauge
    pub head_block: Gauge,
    /// Current finalized block gauge
    pub finalized_block: Gauge,
    /// Cache hit rate gauge
    pub cache_hit_rate: Gauge,
    /// Memory usage gauge
    pub memory_usage: Gauge,
}

impl PerformanceMetrics {
    /// Create new performance metrics
    pub fn new(registry: &Registry) -> ObservabilityResult<Self> {
        let batches_imported = Counter::with_opts(Opts::new("cdk_batches_imported", "Total number of batches imported"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create counter: {}", e)))?;
        
        let batch_import_duration = Histogram::with_opts(
            HistogramOpts::new("cdk_batch_import_duration_seconds", "Duration of batch import operations")
        ).map_err(|e| ObservabilityError::MetricsError(format!("Failed to create histogram: {}", e)))?;
        
        let epochs_processed = Counter::with_opts(Opts::new("cdk_epochs_processed", "Total number of epochs processed"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create counter: {}", e)))?;
        
        let epoch_processing_duration = Histogram::with_opts(
            HistogramOpts::new("cdk_epoch_processing_duration_seconds", "Duration of epoch processing operations")
        ).map_err(|e| ObservabilityError::MetricsError(format!("Failed to create histogram: {}", e)))?;
        
        let finality_checks = Counter::with_opts(Opts::new("cdk_finality_checks", "Total number of finality checks"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create counter: {}", e)))?;
        
        let finality_check_duration = Histogram::with_opts(
            HistogramOpts::new("cdk_finality_check_duration_seconds", "Duration of finality check operations")
        ).map_err(|e| ObservabilityError::MetricsError(format!("Failed to create histogram: {}", e)))?;
        
        let head_block = Gauge::with_opts(Opts::new("cdk_head_block", "Current head block number"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create gauge: {}", e)))?;
        
        let finalized_block = Gauge::with_opts(Opts::new("cdk_finalized_block", "Current finalized block number"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create gauge: {}", e)))?;
        
        let cache_hit_rate = Gauge::with_opts(Opts::new("cdk_cache_hit_rate", "Cache hit rate percentage"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create gauge: {}", e)))?;
        
        let memory_usage = Gauge::with_opts(Opts::new("cdk_memory_usage_bytes", "Memory usage in bytes"))
            .map_err(|e| ObservabilityError::MetricsError(format!("Failed to create gauge: {}", e)))?;

        // Register metrics
        registry.register(Box::new(batches_imported.clone()))?;
        registry.register(Box::new(batch_import_duration.clone()))?;
        registry.register(Box::new(epochs_processed.clone()))?;
        registry.register(Box::new(epoch_processing_duration.clone()))?;
        registry.register(Box::new(finality_checks.clone()))?;
        registry.register(Box::new(finality_check_duration.clone()))?;
        registry.register(Box::new(head_block.clone()))?;
        registry.register(Box::new(finalized_block.clone()))?;
        registry.register(Box::new(cache_hit_rate.clone()))?;
        registry.register(Box::new(memory_usage.clone()))?;

        Ok(Self {
            batches_imported,
            batch_import_duration,
            epochs_processed,
            epoch_processing_duration,
            finality_checks,
            finality_check_duration,
            head_block,
            finalized_block,
            cache_hit_rate,
            memory_usage,
        })
    }

    /// Record batch import
    pub fn record_batch_import(&self, duration: Duration) {
        self.batches_imported.inc();
        self.batch_import_duration.observe(duration.as_secs_f64());
    }

    /// Record epoch processing
    pub fn record_epoch_processing(&self, duration: Duration) {
        self.epochs_processed.inc();
        self.epoch_processing_duration.observe(duration.as_secs_f64());
    }

    /// Record finality check
    pub fn record_finality_check(&self, duration: Duration) {
        self.finality_checks.inc();
        self.finality_check_duration.observe(duration.as_secs_f64());
    }

    /// Update head block
    pub fn update_head_block(&self, block_number: U256) {
        self.head_block.set(block_number.to::<u64>() as f64);
    }

    /// Update finalized block
    pub fn update_finalized_block(&self, block_number: U256) {
        self.finalized_block.set(block_number.to::<u64>() as f64);
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&self, hit_rate: f64) {
        self.cache_hit_rate.set(hit_rate);
    }

    /// Update memory usage
    pub fn update_memory_usage(&self, bytes: u64) {
        self.memory_usage.set(bytes as f64);
    }
}

/// Cache for CDK data
pub struct CdkCache {
    /// Batch cache
    batch_cache: Cache<u64, Batch>,
    /// Epoch cache
    epoch_cache: Cache<u64, Epoch>,
    /// Finality tag cache
    finality_cache: Cache<u64, FinalityTag>,
    /// Cache statistics
    stats: CacheStats,
}

/// Cache statistics
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub inserts: u64,
    pub evictions: u64,
}

impl CacheStats {
    /// Get hit rate
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64 * 100.0
        }
    }
}

impl CdkCache {
    /// Create new CDK cache
    pub fn new(
        batch_capacity: u64,
        epoch_capacity: u64,
        finality_capacity: u64,
        ttl: Duration,
    ) -> Self {
        let batch_cache = Cache::builder()
            .max_capacity(batch_capacity)
            .time_to_live(ttl)
            .build();

        let epoch_cache = Cache::builder()
            .max_capacity(epoch_capacity)
            .time_to_live(ttl)
            .build();

        let finality_cache = Cache::builder()
            .max_capacity(finality_capacity)
            .time_to_live(ttl)
            .build();

        Self {
            batch_cache,
            epoch_cache,
            finality_cache,
            stats: CacheStats::default(),
        }
    }

    /// Get batch from cache
    pub async fn get_batch(&mut self, batch_id: u64) -> Option<Batch> {
        match self.batch_cache.get(&batch_id).await {
            Some(batch) => {
                self.stats.hits += 1;
                debug!("Cache hit for batch {}", batch_id);
                Some(batch)
            }
            None => {
                self.stats.misses += 1;
                debug!("Cache miss for batch {}", batch_id);
                None
            }
        }
    }

    /// Insert batch into cache
    pub async fn insert_batch(&mut self, batch_id: u64, batch: Batch) {
        self.batch_cache.insert(batch_id, batch).await;
        self.stats.inserts += 1;
        debug!("Inserted batch {} into cache", batch_id);
    }

    /// Get epoch from cache
    pub async fn get_epoch(&mut self, epoch_id: u64) -> Option<Epoch> {
        match self.epoch_cache.get(&epoch_id).await {
            Some(epoch) => {
                self.stats.hits += 1;
                debug!("Cache hit for epoch {}", epoch_id);
                Some(epoch)
            }
            None => {
                self.stats.misses += 1;
                debug!("Cache miss for epoch {}", epoch_id);
                None
            }
        }
    }

    /// Insert epoch into cache
    pub async fn insert_epoch(&mut self, epoch_id: u64, epoch: Epoch) {
        self.epoch_cache.insert(epoch_id, epoch).await;
        self.stats.inserts += 1;
        debug!("Inserted epoch {} into cache", epoch_id);
    }

    /// Get finality tag from cache
    pub async fn get_finality_tag(&mut self, batch_id: u64) -> Option<FinalityTag> {
        match self.finality_cache.get(&batch_id).await {
            Some(tag) => {
                self.stats.hits += 1;
                debug!("Cache hit for finality tag {}", batch_id);
                Some(tag)
            }
            None => {
                self.stats.misses += 1;
                debug!("Cache miss for finality tag {}", batch_id);
                None
            }
        }
    }

    /// Insert finality tag into cache
    pub async fn insert_finality_tag(&mut self, batch_id: u64, tag: FinalityTag) {
        self.finality_cache.insert(batch_id, tag).await;
        self.stats.inserts += 1;
        debug!("Inserted finality tag {} into cache", batch_id);
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        self.stats.clone()
    }

    /// Clear all caches
    pub async fn clear(&mut self) {
        self.batch_cache.invalidate_all();
        self.epoch_cache.invalidate_all();
        self.finality_cache.invalidate_all();
        self.stats = CacheStats::default();
        info!("Cleared all caches");
    }

    /// Get cache sizes
    pub async fn get_sizes(&self) -> (usize, usize, usize) {
        let batch_size = self.batch_cache.entry_count() as usize;
        let epoch_size = self.epoch_cache.entry_count() as usize;
        let finality_size = self.finality_cache.entry_count() as usize;
        (batch_size, epoch_size, finality_size)
    }
}

/// Performance monitor for CDK operations
pub struct PerformanceMonitor {
    /// Performance metrics
    metrics: PerformanceMetrics,
    /// CDK cache
    cache: CdkCache,
    /// Start time
    start_time: Instant,
}

impl PerformanceMonitor {
    /// Create new performance monitor
    pub fn new(registry: &Registry) -> ObservabilityResult<Self> {
        let metrics = PerformanceMetrics::new(registry)?;
        let cache = CdkCache::new(
            1000, // batch capacity
            100,  // epoch capacity
            1000, // finality capacity
            Duration::from_secs(3600), // 1 hour TTL
        );

        Ok(Self {
            metrics,
            cache,
            start_time: Instant::now(),
        })
    }

    /// Get metrics reference
    pub fn metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Get cache reference
    pub fn cache(&mut self) -> &mut CdkCache {
        &mut self.cache
    }

    /// Update performance metrics
    pub fn update_metrics(&self) {
        let stats = self.cache.get_stats();
        self.metrics.update_cache_hit_rate(stats.hit_rate());
        
        // Update memory usage (simplified)
        let memory_usage = self.estimate_memory_usage();
        self.metrics.update_memory_usage(memory_usage);
    }

    /// Estimate memory usage
    fn estimate_memory_usage(&self) -> u64 {
        // Simplified memory estimation
        // In a real implementation, this would use proper memory tracking
        let (batch_size, epoch_size, finality_size) = futures::executor::block_on(self.cache.get_sizes());
        (batch_size + epoch_size + finality_size) as u64 * 1024 // Rough estimate
    }

    /// Get uptime
    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Concurrent batch processor
pub struct ConcurrentBatchProcessor {
    /// Number of worker threads
    num_workers: usize,
    /// Batch processing function
    processor: Box<dyn Fn(Batch) -> ObservabilityResult<()> + Send + Sync>,
}

impl ConcurrentBatchProcessor {
    /// Create new concurrent batch processor
    pub fn new<F>(num_workers: usize, processor: F) -> Self
    where
        F: Fn(Batch) -> ObservabilityResult<()> + Send + Sync + 'static,
    {
        Self {
            num_workers,
            processor: Box::new(processor),
        }
    }

    /// Process batches concurrently
    pub fn process_batches(&self, batches: Vec<Batch>) -> ObservabilityResult<Vec<ObservabilityResult<()>>> {
        info!("Processing {} batches with {} workers", batches.len(), self.num_workers);
        
        let results: Vec<ObservabilityResult<()>> = batches
            .par_iter()
            .map(|batch| (self.processor)(batch.clone()))
            .collect();
        
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        info!("Processed {} batches successfully", success_count);
        
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prometheus::Registry;

    #[test]
    fn test_performance_metrics_creation() {
        let registry = Registry::new();
        let metrics = PerformanceMetrics::new(&registry).unwrap();
        
        // Test metric recording
        metrics.record_batch_import(Duration::from_millis(100));
        metrics.record_epoch_processing(Duration::from_millis(200));
        metrics.record_finality_check(Duration::from_millis(50));
        
        metrics.update_head_block(U256::from(1000));
        metrics.update_finalized_block(U256::from(900));
        metrics.update_cache_hit_rate(85.5);
        metrics.update_memory_usage(1024 * 1024);
    }

    #[test]
    fn test_cache_stats() {
        let stats = CacheStats {
            hits: 80,
            misses: 20,
            inserts: 100,
            evictions: 10,
        };
        
        assert_eq!(stats.hit_rate(), 80.0);
    }

    #[tokio::test]
    async fn test_cdk_cache() {
        let mut cache = CdkCache::new(10, 10, 10, Duration::from_secs(60));
        
        // Test batch operations
        let batch = Batch::new(1, vec![]);
        cache.insert_batch(1, batch.clone()).await;
        
        let retrieved = cache.get_batch(1).await;
        assert_eq!(retrieved, Some(batch));
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.inserts, 1);
    }

    #[test]
    fn test_concurrent_batch_processor() {
        let processor = ConcurrentBatchProcessor::new(4, |batch| {
            info!("Processing batch {}", batch.id.number);
            Ok(())
        });
        
        let batches = vec![
            Batch::new(1, vec![]),
            Batch::new(2, vec![]),
            Batch::new(3, vec![]),
        ];
        
        let results = processor.process_batches(batches).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }
}
