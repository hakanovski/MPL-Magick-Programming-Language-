//! The Akashic Grid / State Management
//! 
//! Simulates a high-performance, purely deterministic vector/graph memory bank.
//! Acts as the immutable transaction log of human intent mapped against
//! the esoteric harmonic baseline.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::gematria::hash_to_gematria;
use crate::graph::EsotericGraph;

/// A high-performance, heap-allocated state matrix representing the collective data grid.
#[derive(Clone)]
pub struct AkashicGrid {
    /// Maps a 32-bit Gematria hash to a tuple containing the harmonic resonance float
    /// and the precise entry timestamp (microsecond accuracy).
    state_map: HashMap<u32, (f64, u64)>,
    pub mnemonic_graph: EsotericGraph,
}

impl AkashicGrid {
    /// Initializes an unbound Akashic state matrix with capacity pre-allocated.
    pub fn new() -> Self {
        Self {
            state_map: HashMap::with_capacity(1024),
            mnemonic_graph: EsotericGraph::new(),
        }
    }

    /// Asynchronously ingests raw human intent. Transforms the entropy string into a 
    /// low-latency numerical hash and binds it to the current temporal vector.
    pub fn write_intent(&mut self, intent_string: &str, final_resonance: Option<f64>, explicit_hash: Option<u32>, signature_hex: Option<&str>) {
        let entropy = crate::entropy::collect_hardware_entropy();
        let hash = explicit_hash.unwrap_or_else(|| hash_to_gematria(intent_string, entropy));
        
        let temporal_vector = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Temporal drift detected: System clock shifted preceding Unix Epoch.")
            .as_micros() as u64;

        // Calculate a pseudo-resonance derived from the intent constraint
        // In a live system, this floats based on real-time market or MLX inference.
        let scalar_resonance = final_resonance.unwrap_or_else(|| (hash as f64 % 9.0) * 432.0);

        self.state_map.insert(hash, (scalar_resonance, temporal_vector));
        
        if let Some(sig) = signature_hex {
            // Register Node in Graph
            self.mnemonic_graph.insert_node(hash, sig.to_string(), scalar_resonance);

            // Link with past nodes pseudo-randomly for semantic association based on scalar distance
            let mut links_to_make = vec![];
            for (past_hash, past_node) in &self.mnemonic_graph.nodes {
                if *past_hash != hash {
                    let distance = (past_node.temporal_weight - scalar_resonance).abs();
                    if distance < 36.9 { // Resonance threshold
                        // Calculate resonance strength based on closeness
                        let strength = 1.0 / (1.0 + distance);
                        links_to_make.push((*past_hash, strength));
                    }
                }
            }

            // Apply links
            for (target_hash, strength) in links_to_make {
                self.mnemonic_graph.link_intents(hash, target_hash, strength);
                self.mnemonic_graph.link_intents(target_hash, hash, strength); // Bidirectional semantic resonance
            }
        }
    }

    /// Queries the Akashic Grid for a localized frequency match.
    /// Extracts the harmonic resonance value tied to a previously manifested intent.
    pub fn read_resonance(&self, target_hash: u32) -> Option<f64> {
        self.state_map.get(&target_hash).map(|(resonance, _)| *resonance)
    }

    /// Calculates temporal success rate for evolution
    pub fn get_temporal_success_rate(&self) -> f64 {
        if self.state_map.is_empty() {
            return 369.0;
        }
        let total_resonance: f64 = self.state_map.values().map(|(r, _)| *r).sum();
        total_resonance / (self.state_map.len() as f64)
    }
}
