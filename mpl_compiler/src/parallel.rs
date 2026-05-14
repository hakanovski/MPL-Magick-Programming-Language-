//! Parallel Evolution Engine
//! Executes multiple mutation paths simultaneously across multi-core threads (e.g., Apple M4 Max).

use crate::ast::Program;
use crate::evolution::EvolutionEngine;
use crate::akashic::AkashicGrid;
use rayon::prelude::*;

pub struct ParallelEvolution {
    /// Number of parallel conceptual timelines (threads) to spawn per cycle.
    pub cores: usize,
}

impl ParallelEvolution {
    pub fn new(cores: usize) -> Self {
        Self { cores }
    }

    /// Evaluates multiple mutations concurrently and returns the strongest AST representation.
    pub fn select_prime_resonance(
        &self,
        base_program: &Program,
        base_engine: &EvolutionEngine,
        akashic: &AkashicGrid,
    ) -> (Program, EvolutionEngine) {
        println!("[PARALLEL_EVOLUTION] Spawning {} conceptual timelines...", self.cores);

        let timelines: Vec<usize> = (0..self.cores).collect();
        
        // Rayon parallel iteration over conceptual timelines
        let mut results: Vec<(Program, EvolutionEngine, f64)> = timelines
            .into_par_iter()
            .map(|_id| {
                let mut local_program = base_program.clone();
                let mut local_engine = base_engine.clone();

                // Mutate the timeline (this injects variations driven by true entropy and akashic memory)
                local_engine.mutate_ast(&mut local_program, akashic);
                
                // Fitness simulation (In a fully wired system, we would run `local_program` 
                // in an isolated sandbox and return the `resonance_score`).
                // For now, we simulate a hardware-accelerated fitness score.
                let simulated_fitness = (local_engine.generation as f64 * 432.0) + (crate::entropy::collect_hardware_entropy() as f64 % 369.0);

                (local_program, local_engine, simulated_fitness)
            })
            .collect();

        // Sort by fitness in descending order to find the prime resonance
        results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

        let prime = results.remove(0);
        
        println!("[PARALLEL_EVOLUTION] Prime Resonance acquired with fitness score: {:.1}", prime.2);

        (prime.0, prime.1)
    }
}
