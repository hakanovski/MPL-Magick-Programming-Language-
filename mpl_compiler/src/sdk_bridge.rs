//! The Developer SDK Core
//! Standardizes the response format for external SDKs and Mirror Sandbox execution.

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SdkSimulateResponse {
    pub status: String,
    pub resonance_score: f64,
    pub temporal_resonance: f64,
    pub probability_confidence: f32,
    pub neural_signature: String,
    pub visual_sigil: Option<Vec<crate::sigil::SigilPoint>>,
    pub is_shadow_mode: bool,
    pub message: String,
}

impl SdkSimulateResponse {
    pub fn map_from_ovm(ovm: &crate::ovm::OVM, resonance_score: f64) -> Self {
        Self {
            status: "Simulated".to_string(),
            resonance_score,
            temporal_resonance: ovm.temporal_resonance,
            probability_confidence: ovm.probability_confidence,
            neural_signature: ovm.astral_signature.signature_hex.clone(),
            visual_sigil: ovm.last_visual_sigil.clone(),
            is_shadow_mode: ovm.is_shadow_mode,
            message: "Simulation active. External effects and permanent ledger seals bypassed.".to_string(),
        }
    }
}
