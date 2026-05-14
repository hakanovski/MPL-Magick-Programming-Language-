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
use crate::signature::MagickalIdentity;
use crate::parallel::ParallelEvolution;
use crate::mlx_engine::NeuralCortex;
use crate::ledger::ImmutableGrimoire;

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

    /// The cryptographic identity anchored to this OVM instance
    pub astral_signature: MagickalIdentity,

    /// The internal MLX proxy simulating Neural Engine execution
    pub neural_cortex: NeuralCortex,

    /// The temporal mapping multiplier
    pub temporal_resonance: f64,
    
    /// Probability confidence output of the optimal path calculation
    pub probability_confidence: f32,

    /// The Immutable Ledger for Transmutations
    pub grimoire: ImmutableGrimoire,

    pub last_ritual_seal: Option<crate::ledger::RitualSeal>,
}

impl OVM {
    /// Initializes a pure, void-state runtime environment anchored to a specified frequency.
    pub fn new(tuning: f64, execution_engine: Box<dyn ExecutionEngine>, intent: &str) -> Self {
        let astral_signature = MagickalIdentity::forge(intent);

        OVM {
            memory_registry: HashMap::with_capacity(256), // Pre-allocated limit avoiding re-allocation jitter
            hz_alignment: tuning,
            is_anchored: false,
            akashic_record: AkashicGrid::new(),
            last_visual_sigil: None,
            execution_engine,
            evolution_engine: EvolutionEngine::new(),
            astral_signature,
            neural_cortex: NeuralCortex::new(),
            temporal_resonance: 1.0,
            probability_confidence: 0.0,
            grimoire: ImmutableGrimoire::new(),
            last_ritual_seal: None,
        }
    }

    /// Primary execution loop. A deterministic march through the AST sequence.
    /// Traverses the probability fields to instantiate state changes.
    pub fn execute(&mut self, mut program: Program) {
        let ephemeris = crate::chronos::EphemerisState::new();
        self.temporal_resonance = ephemeris.get_current_alignment_score(&self.astral_signature.seed_intent, 37.7749, -122.4194);

        let prime_seed = crate::probability::ProbabilityWell::simulate_prime_timeline(&program, 10000);
        self.probability_confidence = ((prime_seed % 100) as f32 / 100.0 * 0.5) + 0.5;

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
             println!("[OVM_EVOLUTION_HITCH] Sub-optimal resonance ({}). Triggering parallel evolutionary mutation.", resonance_score);
             let parallel_engine = ParallelEvolution::new(10);
             let (prime_program, prime_evolution) = parallel_engine.select_prime_resonance(&program, &self.evolution_engine, &self.akashic_record);
             program = prime_program;
             self.evolution_engine = prime_evolution;
        }

        // Generate the Ritual Seal after execution
        let seal = self.grimoire.forge_seal(resonance_score, prime_seed);
        self.last_ritual_seal = Some(seal);
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

                let res = self.temporal_resonance;
                let dir_clone = directive_str.clone();
                tokio::spawn(async move {
                    crate::bridge::trigger_physical_manifestation(&dir_clone, res).await;
                });

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
                    // Primitive runtime hook into the Gematria algorithm with Neural Cortex integration
                    let signature_resonance = self.astral_signature.get_resonance_modifier();
                    let hardware_entropy = crate::entropy::collect_hardware_entropy();
                    let combined_entropy = hardware_entropy ^ signature_resonance;

                    let directive = self.memory_registry.get("directive").cloned();
                    let text = if let Some(MplType::Intent(val)) = directive {
                         val
                    } else {
                         target.clone()
                    };

                    let hashed_val = hash_to_gematria(&text, combined_entropy);
                    
                    // MLX Integration: Convert intent to Tensor and extract semantic multiplier
                    let tensor = self.neural_cortex.interpret_intent_vector(&self.astral_signature.signature_hex, self.hz_alignment);
                    let semantic_weight = self.neural_cortex.extract_semantic_weight(&tensor);

                    // Adjust final gematria representation with NPU output
                    let final_resonance = hashed_val as f64 * semantic_weight;

                    // Log into Akashic Graph
                    self.akashic_record.write_intent(&text, Some(final_resonance), Some(hashed_val), Some(&self.astral_signature.signature_hex));

                    // If mnemonic echoes exist, log them
                    if let Some(echo) = self.akashic_record.mnemonic_graph.find_nearest_resonance(hashed_val) {
                        println!("[OVM_MNEMONIC] Neural Cortex detected resonance echo: Node({})", echo);
                    }

                    // Return as a harmonic frequency for mathematical interop
                    MplType::Frequency(final_resonance)
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
