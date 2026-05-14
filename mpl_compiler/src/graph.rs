//! The Mnemonic Graph
//! Tracks "Resonance Edges" between different intents within the Collective Akashic.

use std::collections::HashMap;

#[derive(Clone)]
pub struct IntentNode {
    pub intent_hash: u32,
    pub signature_hex: String,
    pub temporal_weight: f64,
}

#[derive(Clone)]
pub struct ResonanceEdge {
    pub target_hash: u32,
    pub resonance_strength: f64,
}

#[derive(Clone)]
pub struct EsotericGraph {
    pub nodes: HashMap<u32, IntentNode>,
    pub edges: HashMap<u32, Vec<ResonanceEdge>>,
}

impl EsotericGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::with_capacity(1024),
            edges: HashMap::with_capacity(1024),
        }
    }

    pub fn insert_node(&mut self, intent_hash: u32, signature_hex: String, temporal_weight: f64) {
        self.nodes.insert(
            intent_hash,
            IntentNode {
                intent_hash,
                signature_hex,
                temporal_weight,
            },
        );
        self.edges.entry(intent_hash).or_insert_with(Vec::new);
    }

    pub fn link_intents(&mut self, source_hash: u32, target_hash: u32, strength: f64) {
        if let Some(edges) = self.edges.get_mut(&source_hash) {
            // Check if edge already exists to update or push new
            if let Some(edge) = edges.iter_mut().find(|e| e.target_hash == target_hash) {
                edge.resonance_strength = (edge.resonance_strength + strength) / 2.0;
            } else {
                edges.push(ResonanceEdge {
                    target_hash,
                    resonance_strength: strength,
                });
            }
        }
    }

    /// Allows the OVM to "remember" similar past rituals.
    pub fn find_nearest_resonance(&self, current_hash: u32) -> Option<u32> {
        if let Some(edges) = self.edges.get(&current_hash) {
            edges
                .iter()
                .max_by(|a, b| a.resonance_strength.partial_cmp(&b.resonance_strength).unwrap_or(std::cmp::Ordering::Equal))
                .map(|e| e.target_hash)
        } else {
            None
        }
    }
}
