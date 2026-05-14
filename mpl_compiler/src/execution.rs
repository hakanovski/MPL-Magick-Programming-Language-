//! The Execution Engine / Transmutation Layer
//! 
//! Converts abstract virtual machine state changes and intent vectors
//! into tangible reality. Handles hardware IO and market order routing.

use crate::ast::MplType;
use crate::gematria::hash_to_gematria;
use crate::stdlib::inject_entropy_ward;

/// Transmutes determined virtual states into real-world thermodynamic actions.
pub trait ExecutionEngine {
    /// Translates an explicit intent to physical reality.
    fn execute_intent(&self, directive: &str, payload: MplType);
}

/// The baseline executor routing directly to high-throughput financial or physical infrastructure.
pub struct MarketExecutor {}

impl MarketExecutor {
    /// Allocates the infrastructure executor.
    pub fn new() -> Self {
        Self {}
    }
}

impl ExecutionEngine for MarketExecutor {
    fn execute_intent(&self, directive: &str, payload: MplType) {
        let entropy = crate::entropy::collect_hardware_entropy();
        let signature = hash_to_gematria(directive, entropy);
        
        println!("[EXECUTION_LAYER] ⚠️ TRANSMUTATION INITIATED ⚠️");
        println!("[EXECUTION_LAYER] Intent Hash Signature: 0x{:08x}", signature);
        println!("[EXECUTION_LAYER] Directive Vector: \"{}\"", directive);
        match payload {
            MplType::Frequency(val) => println!("[EXECUTION_LAYER] Action Value Anchor (Freq): {:.4}", val),
            _ => println!("[EXECUTION_LAYER] Complex payload routed."),
        }
        println!("[EXECUTION_LAYER] Payload routed to external API execution queue.");
    }
}

/// A general-purpose executor for standard Magick operations and digital/physical warding.
pub struct AppManifestExecutor {}

impl AppManifestExecutor {
    pub fn new() -> Self {
        Self {}
    }
}

impl ExecutionEngine for AppManifestExecutor {
    fn execute_intent(&self, directive: &str, payload: MplType) {
        println!("[APP_EXECUTOR] Parsing directive mapping -> {}", directive);

        if directive == "READ_OUIJA" {
            println!("[APP_EXECUTOR] Processing payload as biometric entropy.");
            println!("[APP_EXECUTOR] Outputting spiritual resonance -> {:?}", payload);
        } else if directive == "CAST_WARD" {
            if let MplType::Intent(target) = payload {
                inject_entropy_ward(&target);
            } else if let MplType::Sigil(target) = payload {
                inject_entropy_ward(&target);
            } else {
                println!("[APP_EXECUTOR] Insufficient target payload type for CAST_WARD.");
            }
        } else {
            // General executor fallback
            let entropy = crate::entropy::collect_hardware_entropy();
            let signature = hash_to_gematria(directive, entropy);
            println!("[APP_EXECUTOR] Abstract transmutation invoked. Sig: 0x{:08x}", signature);
        }
    }
}
