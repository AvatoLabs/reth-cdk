//! HTTP-based batch data source implementation

use crate::{
    Checkpoint, DatastreamError, DatastreamResult, SourceMetadata, BatchSource,
};
use cdk_types::Batch;
use alloy_primitives::U256;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, info};
use url::Url;

/// Configuration for HTTP batch source
#[derive(Debug, Clone)]
pub struct HttpBatchSourceConfig {
    /// Base URL for the batch API
    pub base_url: Url,
    /// API key for authentication (optional)
    pub api_key: Option<String>,
    /// Request timeout
    pub timeout: Duration,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Retry delay
    pub retry_delay: Duration,
}

impl Default for HttpBatchSourceConfig {
    fn default() -> Self {
        Self {
            base_url: Url::parse("http://localhost:8080").unwrap(),
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

/// HTTP-based batch source implementation
#[derive(Debug)]
pub struct HttpBatchSource {
    config: HttpBatchSourceConfig,
    client: Client,
    current_checkpoint: Option<Checkpoint>,
    metadata: SourceMetadata,
}

impl HttpBatchSource {
    /// Create a new HTTP batch source
    pub fn new(config: HttpBatchSourceConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        let metadata = SourceMetadata::new(
            "HTTP Batch Source".to_string(),
            "1.0".to_string(),
            config.base_url.to_string(),
            true,
        );

        Self {
            config,
            client,
            current_checkpoint: None,
            metadata,
        }
    }

    /// Create from URL string
    pub fn from_url(url: &str) -> DatastreamResult<Self> {
        let url = Url::parse(url)
            .map_err(|e| DatastreamError::ConfigError(format!("Invalid URL: {}", e)))?;
        
        let config = HttpBatchSourceConfig {
            base_url: url,
            ..Default::default()
        };

        Ok(Self::new(config))
    }

    /// Make an authenticated request
    async fn make_request(&self, path: &str) -> DatastreamResult<reqwest::Response> {
        let url = self.config.base_url.join(path)
            .map_err(|e| DatastreamError::ConfigError(format!("Invalid path: {}", e)))?;

        let mut request = self.client.get(url);

        if let Some(api_key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request.send().await
            .map_err(|e| DatastreamError::NetworkError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(DatastreamError::HttpError {
                status: response.status().as_u16(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }

    /// Fetch batches from the API
    async fn fetch_batches(&self, from_batch: Option<U256>) -> DatastreamResult<Vec<Batch>> {
        let path = if let Some(batch_id) = from_batch {
            format!("/api/v1/batches?from={}", batch_id)
        } else {
            "/api/v1/batches".to_string()
        };

        let response = self.make_request(&path).await?;
        let batches: Vec<Batch> = response.json().await
            .map_err(|e| DatastreamError::SerializationError(format!("Failed to parse batches: {}", e)))?;

        Ok(batches)
    }

    /// Fetch source metadata
    async fn fetch_metadata(&self) -> DatastreamResult<SourceMetadata> {
        let response = self.make_request("/api/v1/metadata").await?;
        let metadata: SourceMetadata = response.json().await
            .map_err(|e| DatastreamError::SerializationError(format!("Failed to parse metadata: {}", e)))?;

        Ok(metadata)
    }
}

#[async_trait::async_trait]
impl BatchSource for HttpBatchSource {
    async fn next(&mut self) -> DatastreamResult<Option<Batch>> {
        let from_batch = self.current_checkpoint
            .as_ref()
            .map(|cp| cp.last_batch_id + U256::from(1));

        debug!("Fetching batches from: {:?}", from_batch);

        let batches = self.fetch_batches(from_batch).await?;

        if batches.is_empty() {
            debug!("No new batches available");
            return Ok(None);
        }

        let batch = batches.into_iter().next().unwrap();
        
        // Update checkpoint
        self.current_checkpoint = Some(Checkpoint::from_batch(&batch, std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()));

        info!("Fetched batch {} with {} blocks", batch.id.number, batch.block_count());
        Ok(Some(batch))
    }

    async fn checkpoint(&self) -> DatastreamResult<Checkpoint> {
        self.current_checkpoint
            .clone()
            .ok_or_else(|| DatastreamError::CheckpointError("No checkpoint available".to_string()))
    }

    async fn set_checkpoint(&mut self, checkpoint: Checkpoint) -> DatastreamResult<()> {
        debug!("Setting checkpoint to batch {}", checkpoint.last_batch_id);
        self.current_checkpoint = Some(checkpoint);
        Ok(())
    }

    async fn health_check(&self) -> DatastreamResult<()> {
        let response = self.make_request("/api/v1/health").await?;
        
        if response.status().is_success() {
            Ok(())
        } else {
            Err(DatastreamError::SourceUnavailable("Health check failed".to_string()))
        }
    }

    async fn metadata(&self) -> DatastreamResult<SourceMetadata> {
        // Try to fetch fresh metadata, fall back to cached
        match self.fetch_metadata().await {
            Ok(metadata) => Ok(metadata),
            Err(_) => Ok(self.metadata.clone()),
        }
    }

    async fn fetch_batch_stream(&self, _start_batch_number: Option<u64>) -> DatastreamResult<crate::BatchStream> {
        // For HTTP source, we'll return an empty stream for now
        // In a real implementation, this would make HTTP requests to fetch batches
        let stream = async_stream::stream! {
            // Empty stream for now - yield nothing
            if false {
                yield Ok(Batch::new(
                    cdk_types::BatchId::new(U256::ZERO, alloy_primitives::FixedBytes::ZERO),
                    U256::ZERO,
                    alloy_primitives::FixedBytes::ZERO,
                    vec![],
                    cdk_types::ProofMetadata::default(),
                    0,
                ));
            }
        };
        Ok(Box::new(Box::pin(stream)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{FixedBytes, U256};

    #[tokio::test]
    async fn test_http_batch_source_creation() {
        let config = HttpBatchSourceConfig::default();
        let source = HttpBatchSource::new(config);
        
        assert!(source.current_checkpoint.is_none());
        assert_eq!(source.metadata.name, "HTTP Batch Source");
    }

    #[tokio::test]
    async fn test_checkpoint_creation() {
        let checkpoint = Checkpoint::new(
            U256::from(100),
            FixedBytes::from([1u8; 32]),
            U256::from(1000),
            1234567890,
        );

        assert!(checkpoint.is_valid());
        assert_eq!(checkpoint.last_batch_id, U256::from(100));
    }

    #[tokio::test]
    async fn test_memory_checkpoint_storage() {
        let storage = crate::MemoryCheckpointStorage::default();
        let checkpoint = Checkpoint::new(
            U256::from(1),
            FixedBytes::from([1u8; 32]),
            U256::from(100),
            1234567890,
        );

        storage.save_checkpoint(checkpoint.clone()).await.unwrap();
        let loaded = storage.load_checkpoint().await.unwrap();
        
        assert_eq!(loaded, Some(checkpoint));
    }
}
