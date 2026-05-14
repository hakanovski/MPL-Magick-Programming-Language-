//! The Probability Well
//! Monte Carlo simulation layer utilizing Apple Silicon hardware threads to collapse alternate timelines into the Prime execution.

use crate::ast::Program;
use rayon::prelude::*;
use crate::entropy::collect_hardware_entropy;

pub struct ProbabilityWell;

impl ProbabilityWell {
    /// Simulates the AST logic thousands of times to find the optimal high-resonance hardware seed.
    /// In a fully realized system, this mutates OVM inner state, but here we simulate the Monte Carlo math.
    pub fn simulate_prime_timeline(program: &Program, iterations: usize) -> u64 {
        println!("[PROBABILITY_WELL] Initiating Timeline Collapse simulation ({} iterations)...", iterations);

        // We use Rayon to split the workload across CPU cores
        let results: Vec<(u64, f64)> = (0..iterations)
            .into_par_iter()
            .map(|_| {
                // 1. Generate Ghost Entropy
                let ghost_seed = collect_hardware_entropy();
                
                // 2. Simulate AST execution resonance (Pseudo-randomized with AST properties)
                // We fake the deep run via a fast hash algorithm and float modulation to preserve CPU
                let score = Self::ghost_execution_score(program, ghost_seed);
                
                (ghost_seed, score)
            })
            .collect();

        // 3. Find the timeline with the highest resonance
        let prime_timeline = results.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        if let Some((prime_seed, prime_score)) = prime_timeline {
            println!("[PROBABILITY_WELL] Prime Timeline acquired. Seed: 0x{:016x}, Optimal Score: {:.4}", prime_seed, prime_score);
            prime_seed
        } else {
            collect_hardware_entropy()
        }
    }

    /// Fast inner product simulating execution overhead for AST properties.
    fn ghost_execution_score(program: &Program, seed: u64) -> f64 {
        let mut sum = 0.0;
        let seed_f = (seed % 1000) as f64 + 1.0;
        
        for _statement in &program.statements {
            // Incorporate statements size into seed math
            sum += seed_f * 0.001; 
        }

        // Apply a non-linear transform resembling quantum uncertainty
        let resonance = (sum.sin() * 432.0).abs() + (seed as f64 % 369.0);
        resonance
    }
}
