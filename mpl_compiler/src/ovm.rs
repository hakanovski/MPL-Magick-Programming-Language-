//! Occult Virtual Machine (OVM) Execution Layer
//!
//! The deterministic runtime environment for the Magick Programming Language.
//! Translates the AST representations of intent directly into CPU/GPU localized
//! state matrices. Optimized for Apple Silicon unified memory constraints.

use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression, MplType};
use crate::gematria::hash_to_gematria;
use crate::akashic::AkashicGrid;
use crate::stdlib::{invoke_synchronize_mlx, log_to_akashic};

/// The runtime environment holding execution context and bounded memory.
pub struct OVM {
    /// The Akashic Record: A highly optimized, heap-allocated dictionary
    /// linking variable string identifiers to absolute esoteric constants.
    memory_registry: HashMap<String, MplType>,
    
    /// Target harmonic anchor frequency (e.g. 432.0). 
    /// Serves as the scalar baseline for matrix phase transformations.
    hz_alignment: f64,
    
    /// Execution state flag ensuring structural integrity during multi-threaded logic.
    is_anchored: bool,

    /// The collective state grid simulating vector memory constraints.
    akashic_record: AkashicGrid,
}

impl OVM {
    /// Initializes a pure, void-state runtime environment anchored to a specified frequency.
    pub fn new(tuning: f64) -> Self {
        OVM {
            memory_registry: HashMap::with_capacity(256), // Pre-allocated limit avoiding re-allocation jitter
            hz_alignment: tuning,
            is_anchored: false,
            akashic_record: AkashicGrid::new(),
        }
    }

    /// Primary execution loop. A deterministic march through the AST sequence.
    /// Traverses the probability fields to instantiate state changes.
    pub fn execute(&mut self, program: Program) {
        self.is_anchored = true;
        for statement in program.statements {
            self.evaluate_statement(statement);
        }
        self.is_anchored = false;
    }

    /// Evaluates and enforces a specific AST statement primitive.
    fn evaluate_statement(&mut self, statement: Statement) {
        match statement {
            // Sacrifice binds real-world physical boundaries to logical intent.
            Statement::Sacrifice { identifier, value } => {
                let eval_val = self.evaluate_expression(value);
                // The physical memory is mutated, taking ownership of the evaluated constraint.
                self.memory_registry.insert(identifier, eval_val);
            }
            
            // Transmute collapses an intent vector into finalized output / state resolution.
            Statement::Transmute { intent, target } => {
                let eval_val = self.evaluate_expression(intent);
                
                // If the transmuted value is an intent string, serialize it into the ledger.
                if let MplType::Intent(ref text) = eval_val {
                    log_to_akashic(&mut self.akashic_record, text);
                }

                // FFI hook stand-in. In production, this pushes to the MLX layer.
                println!("[OVM_LAYER] Transmutation Event -> Intent Vector collapsed to [{}]: {:?}", target, eval_val);
            }

            // Invocation executes hardcoded macro-functions or triggers Metal shaders.
            Statement::RitualBlock(stmts) => {
                for s in stmts {
                    self.evaluate_statement(s);
                }
            }
        }
    }

    /// Resolves AST Expressions into instantiated memory references or derived scalars.
    fn evaluate_expression(&mut self, expression: Expression) -> MplType {
        match expression {
            Expression::Literal(val) => val,
            Expression::Variable(id) => {
                self.memory_registry
                    .get(&id)
                    .unwrap_or_else(|| panic!("Architectural Fault: Unbound variable / Unmanifested intent -> {}", id))
                    .clone()
            }
            Expression::Invocation { target, .. } => {
                // A primitive dispatcher for esoteric FFI calls or built-ins.
                if target == "synchronize_mlx" {
                    println!("[OVM_LAYER] Harmonic Grid Resonance Invoked. Dispatching MLX Context Lock.");
                    // Synthesizing a simulated harmonic array to pass memory ownership across FFI
                    let temp_matrix = vec![3.0, 6.0, 9.0, 432.0];
                    let mlx_result = invoke_synchronize_mlx(&temp_matrix, self.hz_alignment);
                    MplType::Frequency(mlx_result)
                } else if target == "hash_intent" {
                    // Primitive runtime hook into the Gematria algorithm
                    let hashed_val = hash_to_gematria(&target);
                    // Return as a harmonic frequency for mathematical interop
                    MplType::Frequency(hashed_val as f64)
                } else {
                    panic!("Architectural Fault: Unrecognized external invocation -> {}", target);
                }
            }
            _ => unimplemented!("Complex binary probability logic deferred to Phase 2 Hardware Matrix."),
        }
    }
}
