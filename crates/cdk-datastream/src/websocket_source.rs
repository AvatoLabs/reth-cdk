//! WebSocket data stream source for CDK batch ingestion

use crate::{
    error::{DataStreamError, DataStreamResult},
    source::{BatchSource, BatchStream},
};
use async_trait::async_trait;
use cdk_types::Batch;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};
use url::Url;
use tracing::{debug, info, error};

/// Configuration for the WebSocket batch source
#[derive(Debug, Clone)]
pub struct WebSocketSourceConfig {
    /// The URL of the WebSocket endpoint
    pub url: Url,
}

/// WebSocket implementation of `BatchSource`
#[derive(Debug)]
pub struct WebSocketSource {
    config: WebSocketSourceConfig,
}

impl WebSocketSource {
    /// Create a new WebSocketSource
    pub fn new(config: WebSocketSourceConfig) -> Self {
        Self { config }
    }

    /// Connect to the WebSocket and return the stream
    async fn connect(&self) -> DataStreamResult<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>> {
        info!(target: "cdk::datastream::websocket", url = %self.config.url, "Connecting to WebSocket source");
        let (ws_stream, _) = connect_async(&self.config.url)
            .await
            .map_err(|e| DataStreamError::ConnectionError(format!("Failed to connect to WebSocket: {}", e)))?;
        info!(target: "cdk::datastream::websocket", url = %self.config.url, "WebSocket connection established");
        Ok(ws_stream)
    }
}

#[async_trait]
impl BatchSource for WebSocketSource {
    async fn fetch_batch_stream(&self, _start_batch_number: Option<u64>) -> DataStreamResult<BatchStream> {
        let mut ws_stream = self.connect().await?;

        // For demonstration, we'll just send a subscription message and then
        // simulate receiving batches. In a real scenario, the protocol
        // for requesting and receiving batches would be more complex.
        let subscribe_msg = Message::text(r#"{"jsonrpc":"2.0","method":"cdk_subscribeBatches","params":[],"id":1}"#);
        ws_stream.send(subscribe_msg).await.map_err(|e| DataStreamError::CommunicationError(format!("Failed to send subscription message: {}", e)))?;

        let stream = async_stream::stream! {
            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        debug!(target: "cdk::datastream::websocket", "Received WebSocket message: {}", text);
                        // Attempt to parse the text as a Batch
                        match serde_json::from_str::<Batch>(&text) {
                            Ok(batch) => {
                                info!(target: "cdk::datastream::websocket", batch_number = %batch.id.number, "Received batch from WebSocket");
                                yield Ok(batch);
                            },
                            Err(e) => {
                                error!(target: "cdk::datastream::websocket", error = %e, "Failed to deserialize batch from WebSocket message");
                                yield Err(DataStreamError::DeserializationError(e.to_string()));
                            }
                        }
                    },
                    Ok(Message::Binary(bin)) => {
                        debug!(target: "cdk::datastream::websocket", "Received WebSocket binary message of {} bytes", bin.len());
                        // Attempt to parse binary as a Batch
                        match serde_json::from_slice::<Batch>(&bin) {
                            Ok(batch) => {
                                info!(target: "cdk::datastream::websocket", batch_number = %batch.id.number, "Received batch from WebSocket (binary)");
                                yield Ok(batch);
                            },
                            Err(e) => {
                                error!(target: "cdk::datastream::websocket", error = %e, "Failed to deserialize batch from WebSocket binary message");
                                yield Err(DataStreamError::DeserializationError(e.to_string()));
                            }
                        }
                    },
                    Ok(Message::Ping(p)) => {
                        debug!(target: "cdk::datastream::websocket", "Received WebSocket ping");
                        if let Err(e) = ws_stream.send(Message::Pong(p)).await {
                            error!(target: "cdk::datastream::websocket", error = %e, "Failed to send WebSocket pong");
                            yield Err(DataStreamError::CommunicationError(e.to_string()));
                        }
                    },
                    Ok(Message::Pong(_)) => {
                        debug!(target: "cdk::datastream::websocket", "Received WebSocket pong");
                    },
                    Ok(Message::Close(cf)) => {
                        info!(target: "cdk::datastream::websocket", close_frame = ?cf, "WebSocket connection closed by peer");
                        break;
                    },
                    Ok(Message::Frame(_)) => {
                        // Ignore frame messages
                    },
                    Err(e) => {
                        error!(target: "cdk::datastream::websocket", error = %e, "WebSocket error");
                        yield Err(DataStreamError::ConnectionError(e.to_string()));
                    }
                }
            }
        };
        Ok(Box::new(Box::pin(stream)))
    }

    async fn next(&mut self) -> Result<Option<Batch>, crate::DatastreamError> {
        // For now, return None as we're using the stream-based approach
        Ok(None)
    }

    async fn checkpoint(&self) -> Result<crate::Checkpoint, crate::DatastreamError> {
        Ok(crate::Checkpoint::default())
    }

    async fn set_checkpoint(&mut self, _checkpoint: crate::Checkpoint) -> Result<(), crate::DatastreamError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<(), crate::DatastreamError> {
        // Try to connect to check health
        let _ws_stream = self.connect().await?;
        Ok(())
    }

    async fn metadata(&self) -> Result<crate::SourceMetadata, crate::DatastreamError> {
        Ok(crate::SourceMetadata::new(
            "WebSocket Source".to_string(),
            "1.0".to_string(),
            self.config.url.to_string(),
            true,
        ))
    }
}