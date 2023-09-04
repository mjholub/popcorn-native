use bittorrent::dht::Dht;
use ed25519_dalek::Verifier;
use futures::future;
use hex::FromHex;
use serde_json::from_str;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, Instant};

type Settings = HashMap<String, String>;

async fn update(
    settings: Arc<Settings>,
    e: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(dht_str) = settings.get("dht") {
        let dht = Dht::new(Verifier::from_hex(dht_str).unwrap());
        let hash = FromHex::from_hex(dht_str.as_bytes())?;
        match dht.get(&hash).await {
            Ok(node) => {
                if let Some(node) = node {
                    let new_data = node.v.to_string();
                    let new_info: Result<HashMap<String, String>, _> = from_str(&new_data);
                    match new_info {
                        Ok(info_map) => {
                            // Continue processing
                        }
                        Err(err) => {
                            // Handle the error
                        }
                    }
                }
            }
            Err(_) => {
                // Handle errors
            }
        }
    }

    Ok(())
}
