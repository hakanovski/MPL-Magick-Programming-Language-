//! The Astral Signature
//! Cryptographic Identity & Context Anchor for the OVM.

use crate::entropy::collect_hardware_entropy;
use sha2::{Sha512, Digest};

/// Represents a unique cryptographic identity bound to the hardware and user intent.
#[derive(Clone)]
pub struct MagickalIdentity {
    pub seed_intent: String,
    pub signature_hex: String,
    pub hardware_entropy_baseline: u64,
}

impl MagickalIdentity {
    /// Bootstraps an Astral Signature derived from Hardware Entropy and a seed intent.
    pub fn forge(intent: &str) -> Self {
        let hw_entropy = collect_hardware_entropy();
        
        let mut hasher = Sha512::new();
        hasher.update(intent.as_bytes());
        hasher.update(&hw_entropy.to_le_bytes());
        
        let result = hasher.finalize();
        
        use std::fmt::Write;
        let mut signature_hex = String::with_capacity(result.len() * 2);
        for byte in result {
            write!(&mut signature_hex, "{:02x}", byte).expect("Unable to write hex");
        }

        println!("[KERNEL] Astral Signature 0x{}... Authenticated.", &signature_hex[0..16]);

        Self {
            seed_intent: intent.to_string(),
            signature_hex,
            hardware_entropy_baseline: hw_entropy, // We rely on CSPRNG true entropy injection
        }
    }

    /// Provides a non-linear integer slice of the signature for Gematria injection.
    pub fn get_resonance_modifier(&self) -> u64 {
        let bytes = self.signature_hex.as_bytes();
        let mut modifier: u64 = 0;
        for i in 0..8 {
            if i < bytes.len() {
                modifier ^= (bytes[i] as u64) << (i * 8);
            }
        }
        modifier
    }
}
