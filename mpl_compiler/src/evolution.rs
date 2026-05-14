//! The Self-Modifying Core (Evolution Engine)
//! Implements a feedback loop that evaluates execution fitness
//! and mutates the AST to self-optimize intent manifestation.

use crate::ast::{Expression, Program, Statement, BinaryOperator, MplType};
use rand::Rng;
use crate::akashic::AkashicGrid;

/// Records the outcome of a script execution.
pub struct FitnessRecord {
    pub success: bool,
    pub resonance_score: f64,
}

#[derive(Clone)]
pub struct EvolutionEngine {
    pub generation: usize,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        Self { generation: 1 }
    }

    /// Mutates the AST based on historical fitness scores and collective success vectors.
    pub fn mutate_ast(&mut self, program: &mut Program, akashic: &AkashicGrid) {
        self.generation += 1;
        println!("[EVOLUTION_CORE] Initiating Genetic AST Mutation -> Gen {}", self.generation);

        let temporal_success_rate = akashic.get_temporal_success_rate();
        // The higher the temporal success rate, the less aggressive the basic mutation,
        // but we might pull "Mnemonic Patterns" from successful historical outcomes.
        let mutation_volatility = if temporal_success_rate > 500.0 { 0.02 } else { 0.1 };

        let mut rng = rand::thread_rng();

        for statement in program.statements.iter_mut() {
            if let Statement::Sacrifice { ref mut value, .. } = statement {
                self.mutate_expression(value, &mut rng, mutation_volatility);
            }
        }
    }

    fn mutate_expression(&self, expr: &mut Expression, rng: &mut rand::rngs::ThreadRng, volatility: f64) {
        match expr {
            Expression::Literal(MplType::Frequency(ref mut val)) => {
                // Chance to slightly shift the harmonic frequency based on volatility
                if rng.gen_bool(volatility) {
                    let shift = rng.gen_range(-3.0..3.0);
                    *val += shift;
                    println!("[EVOLUTION_CORE] Mutated Frequency literal by {:.2}", shift);
                }
            }
            Expression::BinaryOp { ref mut op, left, right } => {
                // Chance to alter the resonant binary operation
                let op_volatility = volatility / 2.0;
                if rng.gen_bool(op_volatility) {
                    *op = match rng.gen_range(0..4) {
                        0 => BinaryOperator::HarmonicSum,
                        1 => BinaryOperator::ResonanceMultiply,
                        2 => BinaryOperator::PhaseShift,
                        _ => BinaryOperator::Entangle,
                    };
                    println!("[EVOLUTION_CORE] Mutated BinaryOperator");
                }
                self.mutate_expression(left, rng, volatility);
                self.mutate_expression(right, rng, volatility);
            }
            _ => {}
        }
    }
}
