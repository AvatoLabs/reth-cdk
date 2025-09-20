//! Snapshot validator for data integrity checks

use crate::{SnapResult, SnapError, SnapRecord, SnapMetadata};
use std::path::Path;
use sha2::{Sha256, Digest};
use tokio::fs;

/// Snapshot validator
pub struct SnapValidator;

impl SnapValidator {
    /// Validate snapshot metadata
    pub fn validate_metadata(&self, metadata: &SnapMetadata) -> SnapResult<()> {
        if metadata.version != 1 {
            return Err(SnapError::VersionMismatch {
                expected: 1,
                actual: metadata.version,
            });
        }
        
        if metadata.record_count == 0 {
            return Err(SnapError::Validation("Empty snapshot".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate snapshot file integrity
    pub async fn validate_file(&self, file_path: &Path) -> SnapResult<bool> {
        if !file_path.exists() {
            return Err(SnapError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Snapshot file not found",
            )));
        }
        
        // Check file size
        let metadata = fs::metadata(file_path).await?;
        if metadata.len() == 0 {
            return Err(SnapError::Validation("Empty snapshot file".to_string()));
        }
        
        // Placeholder: calculate and validate checksum
        let content = fs::read(file_path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let checksum = format!("{:x}", hasher.finalize());
        
        tracing::info!("Snapshot file checksum: {}", checksum);
        
        Ok(true)
    }
    
    /// Validate record integrity
    pub fn validate_record(&self, record: &SnapRecord) -> SnapResult<()> {
        if record.key.is_empty() {
            return Err(SnapError::Validation("Empty record key".to_string()));
        }
        
        if record.value.is_empty() {
            return Err(SnapError::Validation("Empty record value".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate batch of records
    pub fn validate_records(&self, records: &[SnapRecord]) -> SnapResult<()> {
        for (i, record) in records.iter().enumerate() {
            self.validate_record(record)
                .map_err(|e| SnapError::Validation(format!("Record {}: {}", i, e)))?;
        }
        
        Ok(())
    }
}
