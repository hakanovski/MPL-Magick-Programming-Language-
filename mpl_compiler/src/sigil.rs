//! The Sigil Engine for Generative Geometry
//! Converts esoteric Gematria hashes into normalized graphical vertex data.

use serde::Serialize;
use std::f32::consts::PI;

/// A geometric vertex mapped to normalized coordinate space (-1.0 to 1.0).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SigilPoint {
    pub x: f32,
    pub y: f32,
}

/// Generates a geometric sigil based on a 32-bit Gematria hash and Hardware Entropy.
/// Implements 3-6-9 harmonic principles for structural routing.
pub fn generate_sigil_data(hash: u32, entropy: u64) -> Vec<SigilPoint> {
    println!("[SIGIL_ENGINE] Synthesizing generative geometry from hash: 0x{:08x} / Entropy: {}", hash, entropy);
    
    // Modify hash using entropy
    let hash = hash ^ (entropy as u32);
    
    // Use the hash digits to dictate the number of vertices and the rotational phasing.
    // 3-6-9 constraints: fallback to numbers divisible by 3.
    let root_val = (hash % 9) as usize;
    let num_points = if root_val == 0 { 9 } else { root_val * 3 }; // guarantees 3, 6, 9, etc.
    
    let mut points = Vec::with_capacity(num_points);
    
    let phi = 1.61803398875_f32;
    // Base radius constrained between 0.3 and 0.8
    let base_radius = 0.3 + (((hash % 100) as f32) / 100.0) * 0.5;
    
    for i in 0..num_points {
        let angle = (i as f32) * (2.0 * PI / (num_points as f32));
        
        // Harmonic distortion tied to Tesla's numbers
        let distortion = (hash.wrapping_mul((i + 1) as u32) % 369) as f32 / 369.0;
        let r = (base_radius + (distortion / phi)).clamp(0.1, 1.0);
        
        points.push(SigilPoint {
            x: r * angle.cos(),
            y: r * angle.sin(),
        });
    }
    
    println!("[SIGIL_ENGINE] Generated {} harmonic vertices.", num_points);
    points
}
