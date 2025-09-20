//! Filesystem data stream source for CDK batch ingestion

use crate::{
    error::{DataStreamError, DataStreamResult},
    source::{BatchSource, BatchStream},
};
use async_trait::async_trait;
use cdk_types::Batch;
use std::{
    path::PathBuf,
};
use tokio::{fs, io::AsyncReadExt};
use futures::{stream, StreamExt};
use tracing::{debug, info, error};

/// Configuration for the Filesystem batch source
#[derive(Debug, Clone)]
pub struct FilesystemSourceConfig {
    /// The directory to read batch files from
    pub path: PathBuf,
    /// File extension to look for (e.g., "json", "rlp")
    pub file_extension: String,
}

/// Filesystem implementation of `BatchSource`
#[derive(Debug)]
pub struct FilesystemSource {
    config: FilesystemSourceConfig,
}

impl FilesystemSource {
    /// Create a new FilesystemSource
    pub fn new(config: FilesystemSourceConfig) -> Self {
        Self { config }
    }

    /// Read a batch from a file
    async fn read_batch_from_file(file_path: PathBuf) -> DataStreamResult<Batch> {
        debug!(target: "cdk::datastream::filesystem", path = %file_path.display(), "Reading batch from file");
        let mut file = fs::File::open(&file_path)
            .await
            .map_err(|e| DataStreamError::IoError(format!("Failed to open file {}: {}", file_path.display(), e)))?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .await
            .map_err(|e| DataStreamError::IoError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        // Assuming batches are stored as JSON for now
        let batch: Batch = serde_json::from_slice(&contents)
            .map_err(|e| DataStreamError::DeserializationError(format!("Failed to deserialize batch from {}: {}", file_path.display(), e)))?;

        info!(target: "cdk::datastream::filesystem", batch_number = %batch.id.number, path = %file_path.display(), "Successfully read batch from file");
        Ok(batch)
    }
}

#[async_trait]
impl BatchSource for FilesystemSource {
    async fn fetch_batch_stream(&self, start_batch_number: Option<u64>) -> DataStreamResult<BatchStream> {
        info!(target: "cdk::datastream::filesystem", path = %self.config.path.display(), start_batch_number = ?start_batch_number, "Fetching batch stream from filesystem");

        let mut entries = fs::read_dir(&self.config.path)
            .await
            .map_err(|e| DataStreamError::IoError(format!("Failed to read directory {}: {}", self.config.path.display(), e)))?;

        let mut file_paths = Vec::new();
        while let Some(entry) = entries.next_entry().await.map_err(|e| DataStreamError::IoError(format!("Failed to read directory entry: {}", e)))? {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext.to_string_lossy() == self.config.file_extension) {
                file_paths.push(path);
            }
        }

        file_paths.sort_unstable(); // Ensure consistent order

        let stream = stream::iter(file_paths)
            .filter_map(move |file_path| {
                let start_batch_number = start_batch_number;
                async move {
                    // Extract batch number from filename or content if needed for filtering
                    // For simplicity, we'll just read all and filter later if start_batch_number is provided
                    match Self::read_batch_from_file(file_path).await {
                        Ok(batch) => {
                            if let Some(start_num) = start_batch_number {
                                if batch.id.number >= start_num {
                                    Some(Ok(batch))
                                } else {
                                    None
                                }
                            } else {
                                Some(Ok(batch))
                            }
                        },
                        Err(e) => {
                            error!(target: "cdk::datastream::filesystem", error = %e, "Failed to read batch file");
                            Some(Err(e))
                        }
                    }
                }
            })
            .boxed();

        Ok(Box::new(stream))
    }

    async fn next(&mut self) -> Result<Option<Batch>, crate::DatastreamError> {
        Ok(None)
    }

    async fn checkpoint(&self) -> Result<crate::Checkpoint, crate::DatastreamError> {
        Ok(crate::Checkpoint::default())
    }

    async fn set_checkpoint(&mut self, _checkpoint: crate::Checkpoint) -> Result<(), crate::DatastreamError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<(), crate::DatastreamError> {
        // Check if the directory exists and is readable
        if !self.config.path.exists() {
            return Err(crate::DatastreamError::IoError(format!("Directory does not exist: {}", self.config.path.display())));
        }
        if !self.config.path.is_dir() {
            return Err(crate::DatastreamError::IoError(format!("Path is not a directory: {}", self.config.path.display())));
        }
        Ok(())
    }

    async fn metadata(&self) -> Result<crate::SourceMetadata, crate::DatastreamError> {
        Ok(crate::SourceMetadata::new(
            "Filesystem Source".to_string(),
            "1.0".to_string(),
            self.config.path.to_string_lossy().to_string(),
            true,
        ))
    }
}