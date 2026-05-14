//! The Self-Modifying Core (Evolution Engine)
//! Implements a feedback loop that evaluates execution fitness
//! and mutates the AST to self-optimize intent manifestation.

use crate::ast::{Expression, Program, Statement, BinaryOperator, MplType};
use rand::Rng;

/// Records the outcome of a script execution.
pub struct FitnessRecord {
    pub success: bool,
    pub resonance_score: f64,
}

pub struct EvolutionEngine {
    pub generation: usize,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        Self { generation: 1 }
    }

    /// Mutates the AST based on historical fitness scores.
    pub fn mutate_ast(&mut self, program: &mut Program) {
        self.generation += 1;
        println!("[EVOLUTION_CORE] Initiating Genetic AST Mutation -> Gen {}", self.generation);

        let mut rng = rand::thread_rng();

        for statement in program.statements.iter_mut() {
            if let Statement::Sacrifice { ref mut value, .. } = statement {
                self.mutate_expression(value, &mut rng);
            }
        }
    }

    fn mutate_expression(&self, expr: &mut Expression, rng: &mut rand::rngs::ThreadRng) {
        match expr {
            Expression::Literal(MplType::Frequency(ref mut val)) => {
                // 10% chance to slightly shift the harmonic frequency
                if rng.gen_bool(0.1) {
                    let shift = rng.gen_range(-3.0..3.0);
                    *val += shift;
                    println!("[EVOLUTION_CORE] Mutated Frequency literal by {:.2}", shift);
                }
            }
            Expression::BinaryOp { ref mut op, left, right } => {
                // 5% chance to alter the resonant binary operation
                if rng.gen_bool(0.05) {
                    *op = match rng.gen_range(0..4) {
                        0 => BinaryOperator::HarmonicSum,
                        1 => BinaryOperator::ResonanceMultiply,
                        2 => BinaryOperator::PhaseShift,
                        _ => BinaryOperator::Entangle,
                    };
                    println!("[EVOLUTION_CORE] Mutated BinaryOperator");
                }
                self.mutate_expression(left, rng);
                self.mutate_expression(right, rng);
            }
            _ => {}
        }
    }
}
