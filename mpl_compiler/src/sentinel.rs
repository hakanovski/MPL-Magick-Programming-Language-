//! The Sovereign Sentinel
//! Autonomous monitoring and intent generation.

use std::time::Duration;
use tokio::time::sleep;

use crate::chronos::EphemerisState;
use crate::oracle::FinancialOracle;
use crate::mlx_engine::NeuralCortex;
use crate::ovm::{OVM, AppManifestExecutor};
use crate::lexer::Lexer;
use crate::parser::Parser;

/// Autonomous loop that monitors conditions and executes unprompted manifestation when resonance peaks.
pub async fn watch_sentinel_windows() {
    println!("[SENTINEL] Sovereign Sentinel online. Scanning the Chronos and Oracle streams for Golden Resonance...");

    let mut oracle = FinancialOracle::new();
    
    loop {
        // Step 1: Check Temporal Resonance
        let ephemeris = EphemerisState::new();
        let current_temporal_score = ephemeris.get_current_alignment_score("SOVEREIGN_SCAN", 37.7749, -122.4194);
        
        // Step 2: Check Oracle Anomaly
        let oracle_state = oracle.fetch_market_volatility().await;
        
        if current_temporal_score > 1.5 && oracle_state.anomaly_detected {
            println!("[SENTINEL] ⚡ GOLDEN RESONANCE DETECTED! Score: {:.3}. Autonomously casting intent...", current_temporal_score);
            
            // Generate intent autonomously
            let intent_text = "Stabilize timeline and extract maximum strategic advantage from anomalous volatility.";
            
            let neural_cortex = NeuralCortex::new();
            let generated_script = neural_cortex.transcode_intent_to_script(intent_text);
            
            println!("[SENTINEL] Generated Script via Neural Cortex:\n{}", generated_script);
            
            let lexer = Lexer::new(&generated_script);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            
            let mut ovm = OVM::new(432.0, Box::new(AppManifestExecutor::new()), intent_text);
            ovm.execute(program);
            
            if let Some(seal) = &ovm.last_ritual_seal {
                println!("[SENTINEL] Sovereign execution sealed. ID: {}", seal.seal_id);
            }
            
            // Sleep longer after a massive autonomous cast to prevent flooding real-world IoT
            sleep(Duration::from_secs(369)).await;
        } else {
            // Keep scanning, relatively frequently
            sleep(Duration::from_secs(10)).await;
        }
    }
}
