//! The Immutable Seal (Merkle-Ledger)
//! Cryptographic persistence of transmutation sequences.

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

/// An immutable cryptographic seal verifying a successful semantic execution.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct RitualSeal {
    pub seal_id: String,
    pub timestamp: u64,
    pub resonance_score: f64,
    pub merkle_root: String,
}

/// The local cryptographic ledger storing all Ritual Seals.
pub struct ImmutableGrimoire {
    seals: HashMap<String, RitualSeal>,
}

impl ImmutableGrimoire {
    pub fn new() -> Self {
        Self {
            seals: HashMap::new(),
        }
    }

    /// Forges a new seal anchoring the physical results of an OVM execution.
    pub fn forge_seal(&mut self, resonance_score: f64, program_hash: u64) -> RitualSeal {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Simulate a SHA-256 identifier and Merkle Root
        let merkle_root = format!("{:016x}{:016x}", program_hash, timestamp);
        let seal_id = format!("seal_{:032x}", program_hash ^ (resonance_score as u64));

        let seal = RitualSeal {
            seal_id: seal_id.clone(),
            timestamp,
            resonance_score,
            merkle_root,
        };

        println!(
            "[IMMUTABLE_SEAL] Forged: Node[{}], Merkle Root: {}, Resonance: {:.3}",
            seal.seal_id, seal.merkle_root, seal.resonance_score
        );

        self.seals.insert(seal_id, seal.clone());

        seal
    }

    pub fn get_seal(&self, seal_id: &str) -> Option<&RitualSeal> {
        self.seals.get(seal_id)
    }
}
