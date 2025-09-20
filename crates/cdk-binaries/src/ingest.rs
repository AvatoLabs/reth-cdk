//! Ingest command implementation

use clap::Parser;
use anyhow::Result;
use cdk_datastream::{BatchSource, HttpBatchSource, HttpBatchSourceConfig};
use cdk_ingest::{MemoryMappingStorage, MappingStorage};
use cdk_observe::{CdkMetrics, CdkTracing};
use std::time::{Instant, Duration};
use url::Url;

/// Ingest batches from data source into Reth
#[derive(Parser)]
#[command(about = "Ingest batches from data source into Reth")]
pub struct IngestCommand {
    /// Data source URL
    #[arg(long, default_value = "http://localhost:8080/batches")]
    pub datastream: String,
    
    /// Starting checkpoint (auto, latest, or specific checkpoint)
    #[arg(long, default_value = "auto")]
    pub from_checkpoint: String,
    
    /// Reth RPC URL
    #[arg(long, default_value = "http://localhost:8545")]
    pub reth_rpc: String,
    
    /// Maximum number of batches to process (0 = unlimited)
    #[arg(long, default_value = "0")]
    pub max_batches: u64,
    
    /// Enable metrics collection
    #[arg(long, default_value = "true")]
    pub enable_metrics: bool,
}

impl IngestCommand {
    pub async fn run(&self) -> Result<()> {
        tracing::info!("Starting CDK ingest process");
        tracing::info!("Data source: {}", self.datastream);
        tracing::info!("Reth RPC: {}", self.reth_rpc);
        tracing::info!("Max batches: {}", self.max_batches);

        // Initialize metrics
        let metrics = CdkMetrics::new();
        
        // Create data source
        let config = HttpBatchSourceConfig {
            base_url: Url::parse(&self.datastream)?,
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        };
        let mut batch_source = HttpBatchSource::new(config);
        
        // Create mapping storage
        let mapping_storage = MemoryMappingStorage::default();
        
        // Create block assembler (simplified - placeholder)
        // let assembler = BlockAssembler::default();
        
        // Process batches
        let mut processed_count = 0;
        let start_time = Instant::now();
        
        loop {
            if self.max_batches > 0 && processed_count >= self.max_batches {
                tracing::info!("Reached maximum batch limit: {}", self.max_batches);
                break;
            }

            match batch_source.next().await {
                Ok(Some(batch)) => {
                    let batch_start = Instant::now();
                    
                    CdkTracing::log_ingestion_start(batch.id.number, batch.blocks.len());
                    
                    // Assemble blocks (simplified - placeholder)
                    // let _block_inputs = assembler.assemble(&batch)?;
                    
                    // Store mappings (simplified)
                    let batch_mapping = cdk_ingest::BatchMapping {
                        batch_id: batch.id.number.to(),
                        batch_hash: batch.id.hash,
                        start_block: 0, // Simplified
                        end_block: batch.blocks.len() as u64,
                        block_count: batch.blocks.len() as u32,
                        epoch_id: 0, // Simplified
                        timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                    };
                    mapping_storage.save_batch_mapping(batch_mapping).await?;
                    
                    // Update metrics
                    metrics.update_batch_height(batch.id.number);
                    metrics.update_ingest_tps(1.0 / batch_start.elapsed().as_secs_f64());
                    
                    let duration_ms = batch_start.elapsed().as_millis() as u64;
                    CdkTracing::log_ingestion_complete(batch.id.number, duration_ms);
                    
                    processed_count += 1;
                    
                    tracing::info!("Processed batch {} ({} blocks) in {}ms", 
                        batch.id.number, batch.blocks.len(), duration_ms);
                }
                Ok(None) => {
                    tracing::info!("No more batches available");
                    break;
                }
                Err(e) => {
                    tracing::error!("Failed to fetch batch: {}", e);
                    metrics.increment_error_count();
                    
                    // Wait before retrying
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
        
        let total_duration = start_time.elapsed();
        tracing::info!("Ingest completed: {} batches processed in {:?}", 
            processed_count, total_duration);
        
        Ok(())
    }
}
