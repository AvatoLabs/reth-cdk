//! Common utilities for CDK binaries

use anyhow::Result;
use std::time::Duration;

/// Parse checkpoint string into Checkpoint
pub fn parse_checkpoint(checkpoint_str: &str) -> Result<Option<cdk_datastream::Checkpoint>> {
    match checkpoint_str {
        "auto" => Ok(None),
        "latest" => Ok(None),
        _ => {
            // Try to parse as specific checkpoint
            // This is a placeholder - in real implementation, you'd parse the checkpoint format
            tracing::warn!("Specific checkpoint parsing not implemented, using auto");
            Ok(None)
        }
    }
}

/// Validate URL format
pub fn validate_url(url: &str) -> Result<()> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid URL format: {}", url))
    }
}

/// Create a retry delay with exponential backoff
pub fn retry_delay(attempt: u32) -> Duration {
    let base_delay = Duration::from_secs(1);
    let max_delay = Duration::from_secs(60);
    
    let delay = base_delay * 2_u32.pow(attempt.min(6));
    delay.min(max_delay)
}

/// Format duration for human reading
pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes % 60, seconds % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_checkpoint() {
        assert!(parse_checkpoint("auto").unwrap().is_none());
        assert!(parse_checkpoint("latest").unwrap().is_none());
    }

    #[test]
    fn test_validate_url() {
        assert!(validate_url("http://localhost:8080").is_ok());
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("invalid-url").is_err());
    }

    #[test]
    fn test_retry_delay() {
        let delay1 = retry_delay(0);
        let delay2 = retry_delay(1);
        let delay3 = retry_delay(10);
        
        assert!(delay2 > delay1);
        assert_eq!(delay3, Duration::from_secs(60)); // Max delay
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(Duration::from_secs(3661)), "1h 1m 1s");
    }
}
