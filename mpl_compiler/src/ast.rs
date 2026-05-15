//! AST (Abstract Syntax Tree) for the Magick Programming Language (MPL).
//! 
//! This module defines the strict, memory-safe data structures that represent
//! the esoteric constraints and probabilistic calculations of the MPL runtime.
//! All types are deterministic to ensure flawless execution on Apple Silicon.

#[derive(Debug, PartialEq, Clone)]
pub enum MplType {
    /// Cryptographic/symbolic representation of an entity or concept.
    /// Used for precise addressing in the Occult Virtual Machine.
    Sigil(String),               
    
    /// Harmonic frequency scalar (e.g., 3.0, 6.0, 9.0).
    /// Used as a multiplier in super-positioning matrix operations.
    Frequency(f64),              
    
    /// Immutable plain-text vector representing human intent.
    /// Acts as the localized seed for Gematria hashing.
    Intent(String),              
    
    /// Probabilistic matrix constraint for tensor calculations on Apple MLX.
    TensorGate(Vec<f64>),        

    /// Visual generative geometry representing an esoteric sigil.
    VisualSigil(Vec<crate::sigil::SigilPoint>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// A raw esoteric constant.
    Literal(MplType),
    
    /// A pointer to an enshrined value in the Akashic Record (Memory Registry).
    Variable(String),
    
    /// Binary operation executed over fields of probability.
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    
    /// A call to a deeper ritual or C++/Metal FFI layer.
    Invocation {
        target: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    /// Harmonic Resonance (`+`), tailored for the frequency domain.
    HarmonicSum,       
    
    /// Resonance Multiplication (`*`), tailored for super-positioning matrices.
    ResonanceMultiply, 
    
    /// Phase Shifting (`>`), an entropic inequality check.
    PhaseShift,        
    
    /// Quantum Entanglement (`==`), a strict state equality evaluation.
    Entangle,          
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Variable Declaration / Assignment. Binds real-world physical cost (Sacrifice)
    /// to a specific logical identifier in the registry.
    Sacrifice { identifier: String, value: Expression }, 
    
    /// State Change / Output. Forces an intent vector to collapse into a target state.
    Transmute { intent: Expression, target: String },    
    
    /// Scoped execution block, ensuring local variance doesn't pollute the global state.
    RitualBlock(Vec<Statement>),                         
    
    /// Dynamically fetches and anchors an external esoteric context.
    Import { package: String },
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    /// The linear sequence of esoteric operations representing the final ritual.
    pub statements: Vec<Statement>,
}
