//! Occult Virtual Machine (OVM)
//! Deterministic runtime execution for the resulting AST. Designed for
//! memory safety and zero-cost abstraction before pushing to GPU Compute.

use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression, MplType};

pub struct OccultVM {
    /// State variables (The Akashic Record)
    memory_registry: HashMap<String, MplType>,
    hz_alignment: f64, // Base tuning frequency
}

impl OccultVM {
    pub fn new(tuning: f64) -> Self {
        OccultVM {
            memory_registry: HashMap::new(),
            hz_alignment: tuning,
        }
    }

    /// Primary execution loop. Translates intention vectors to outcomes.
    pub fn execute(&mut self, program: Program) {
        for statement in program.statements {
            self.evaluate_statement(statement);
        }
    }

    fn evaluate_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Sacrifice { identifier, value } => {
                let eval_val = self.evaluate_expression(value);
                self.memory_registry.insert(identifier, eval_val);
            }
            Statement::Transmute { intent, target } => {
                let eval_val = self.evaluate_expression(intent);
                println!(">> TRANSMUTATION COMPLETE -> TARGET: {} | ANOMALY MATRIX: {:?}", target, eval_val);
            }
            Statement::RitualBlock(stmts) => {
                for s in stmts {
                    self.evaluate_statement(s);
                }
            }
        }
    }

    fn evaluate_expression(&self, expression: Expression) -> MplType {
        match expression {
            Expression::Literal(val) => val,
            Expression::Variable(id) => {
                self.memory_registry.get(&id).expect("Variable not manifested in memory").clone()
            }
            _ => unimplemented!("Complex binary and invocation logic routing to Apple Metal APIs"),
        }
    }
}
