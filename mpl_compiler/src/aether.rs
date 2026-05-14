//! The Aetheric Network (API Abstraction)
//! 
//! The gateway to the external universe. This module provides asynchronous 
//! telemetry ingestion from distributed nodes, acting as the sensor array 
//! for the Oracle daemon.

use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

/// A provider-agnostic interface for ingesting physical market entropy.
#[async_trait::async_trait]
pub trait AetherSource {
    /// Streams live telemetry from an external source, firing the callback upon each tick.
    async fn stream_ticks(
        &self,
        pair: &str,
        mut callback: Box<dyn FnMut(f64) + Send + 'static>,
    ) -> Result<(), String>;
}

/// The ultra-low latency Kraken WebSocket provider.
pub struct KrakenProvider {}

impl KrakenProvider {
    /// Bootstraps a new Kraken instance.
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl AetherSource for KrakenProvider {
    async fn stream_ticks(
        &self,
        pair: &str,
        mut callback: Box<dyn FnMut(f64) + Send + 'static>,
    ) -> Result<(), String> {
        let kraken_ws_url = "wss://ws.kraken.com";
        println!("[AETHER] Engaging live stream interface to {}", kraken_ws_url);

        let (mut ws_stream, _) = connect_async(kraken_ws_url)
            .await
            .map_err(|e| format!("Failed to connect to Kraken WebSocket: {}", e))?;
            
        println!("[AETHER] WebSocket uplink established. Handshaking...");

        let subscribe_payload = serde_json::json!({
            "event": "subscribe",
            "pair": [pair],
            "subscription": {
                "name": "trade"
            }
        });

        ws_stream
            .send(Message::Text(subscribe_payload.to_string().into()))
            .await
            .map_err(|e| format!("Failed to send subscribe message: {}", e))?;

        println!("[AETHER] Subscribed to {} trade stream. Ingesting raw ticks...", pair);

        while let Some(msg_result) = ws_stream.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    let parsed: Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };

                    if parsed.is_array() {
                        if let Some(array) = parsed.as_array() {
                            if array.len() >= 2 {
                                if let Some(trades) = array[1].as_array() {
                                    for trade_item in trades {
                                        if let Some(trade) = trade_item.as_array() {
                                            if let Some(price_str) = trade.get(0).and_then(|v| v.as_str()) {
                                                if let Ok(price) = price_str.parse::<f64>() {
                                                    callback(price);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    return Err("Kraken closed the connection.".to_string());
                }
                Err(e) => {
                    return Err(format!("WebSocket streaming error: {}", e));
                }
                _ => {}
            }
        }

        Err("Aether stream disconnected unexpectedly.".to_string())
    }
}
