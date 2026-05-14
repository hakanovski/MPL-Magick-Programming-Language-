//! The External Physical Bridge (IoT Synthesis)
//! Fires webhooks to alter the physical environment based on the ritual's resonance.

use serde_json::json;

pub async fn trigger_physical_manifestation(directive: &str, resonance: f64) {
    if resonance < 1.0 {
        return; // Only trigger for highly resonant transmutations
    }

    println!("[EXTERNAL_BRIDGE] Activating physical environment for directive: {}", directive);

    // In a real deployment, this would be loaded from an environment variable.
    let webhook_url = std::env::var("MPL_WEBHOOK_URL").unwrap_or_else(|_| "http://localhost:8080/manifest_hook".to_string());

    let payload = json!({
        "directive": directive,
        "resonance": resonance,
        "action": "synthesize_environment",
    });

    let client = reqwest::Client::new();
    match client.post(&webhook_url)
        .json(&payload)
        .send()
        .await 
    {
        Ok(res) => {
            if res.status().is_success() {
                println!("[EXTERNAL_BRIDGE] SUCCESS. Physical environment has been altered.");
            } else {
                println!("[EXTERNAL_BRIDGE] WARNING. Webhook received non-success status: {}", res.status());
            }
        }
        Err(e) => {
            println!("[EXTERNAL_BRIDGE] ERROR. Could not bridge to physical world: {}", e);
        }
    }
}
