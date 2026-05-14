//! Occult Virtual Machine (OVM) Execution Layer
//!
//! The deterministic runtime environment for the Magick Programming Language.
//! Translates the AST representations of intent directly into CPU/GPU localized
//! state matrices. Optimized for Apple Silicon unified memory constraints.

use std::collections::HashMap;
use crate::ast::{Program, Statement, Expression, MplType};
use crate::benchmark::Oracle; // Keep standard imports if any
use crate::gematria::hash_to_gematria;
use crate::akashic::AkashicGrid;
use crate::stdlib::{invoke_synchronize_mlx, log_to_akashic};
use crate::execution::ExecutionEngine;
use crate::evolution::EvolutionEngine;

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

    /// State capture for external API returning geometric intent
    pub last_visual_sigil: Option<Vec<crate::sigil::SigilPoint>>,

    /// The execution engine for transmuting intent into physical reality
    execution_engine: Box<dyn ExecutionEngine>,

    /// The Self-Modifying Core handling programmatic mutations
    pub evolution_engine: EvolutionEngine,
}

impl OVM {
    /// Initializes a pure, void-state runtime environment anchored to a specified frequency.
    pub fn new(tuning: f64, execution_engine: Box<dyn ExecutionEngine>) -> Self {
        OVM {
            memory_registry: HashMap::with_capacity(256), // Pre-allocated limit avoiding re-allocation jitter
            hz_alignment: tuning,
            is_anchored: false,
            akashic_record: AkashicGrid::new(),
            last_visual_sigil: None,
            execution_engine,
            evolution_engine: EvolutionEngine::new(),
        }
    }

    /// Primary execution loop. A deterministic march through the AST sequence.
    /// Traverses the probability fields to instantiate state changes.
    pub fn execute(&mut self, mut program: Program) {
        self.is_anchored = true;
        
        // Execute the current generation of the AST
        for statement in program.statements.clone() {
            self.evaluate_statement(statement);
        }
        self.is_anchored = false;

        // Simulate reading the Akashic Record outcome for fitness.
        // In a true deployment, this would trace actual physical entropy/market results.
        let resonance_score = self.akashic_record.get_temporal_success_rate();
        let is_successful = resonance_score >= 369.0; // placeholder for hardware feedback loop

        if !is_successful || self.evolution_engine.generation < 3 {
             println!("[OVM_EVOLUTION_HITCH] Sub-optimal resonance ({}). Triggering evolutionary mutation.", resonance_score);
             self.evolution_engine.mutate_ast(&mut program, &self.akashic_record);
        }
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
            Statement::Transmute { intent, target: _ } => {
                let eval_val = self.evaluate_expression(intent);
                
                let directive_str = if let MplType::Intent(ref text) = eval_val {
                    text.clone()
                } else {
                    "UNKNOWN_DIRECTIVE".to_string()
                };

                // If the transmuted value is an intent string, serialize it into the ledger.
                if let MplType::Intent(ref text) = eval_val {
                    log_to_akashic(&mut self.akashic_record, text);
                }

                // Resolving standard esoteric payload from the grid
                let payload = self.memory_registry.get("target_vector").cloned().unwrap_or(eval_val.clone());

                // Capture visual geometry if present
                if let MplType::VisualSigil(ref data) = payload {
                    self.last_visual_sigil = Some(data.clone());
                } else if let MplType::VisualSigil(ref data) = eval_val {
                    self.last_visual_sigil = Some(data.clone());
                }

                // Transmute into physical reality via hardware/market hooks
                self.execution_engine.execute_intent(&directive_str, payload.clone());

                // FFI hook stand-in. In production, this pushes to the MLX layer.
                println!("[OVM_LAYER] Transmutation Event -> Intent Vector collapsed to [{}]: {:?}", directive_str, payload);
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
                    let entropy = crate::entropy::collect_hardware_entropy();
                    let hashed_val = hash_to_gematria(&target, entropy);
                    // Return as a harmonic frequency for mathematical interop
                    MplType::Frequency(hashed_val as f64)
                } else if target == "generate_sigil" {
                    // Evaluate the intent or target, for now we will just use a placeholder intent or extract it
                    // if args were parsed, but ast expression doesn't resolve args inside dispatch yet.
                    // We will simulate it using a register.
                    let directive = self.memory_registry.get("directive")
                        .or_else(|| self.memory_registry.get("target_vector"))
                        .cloned();
                        
                    let text = if let Some(MplType::Intent(val)) = directive {
                         val
                    } else if let Some(MplType::Sigil(val)) = directive {
                         val
                    } else {
                         "UNIVERSAL_HARMONIC".to_string()
                    };
                    
                    let sigil_data = crate::stdlib::invoke_generate_sigil(&text);
                    MplType::VisualSigil(sigil_data)
                } else {
                    panic!("Architectural Fault: Unrecognized external invocation -> {}", target);
                }
            }
            _ => unimplemented!("Complex binary probability logic deferred to Phase 2 Hardware Matrix."),
        }
    }
}
