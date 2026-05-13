mod lexer;
mod parser;
mod ast;
mod ovm;
mod gematria;
mod harmonic;
mod akashic;
mod metal_ffi;
mod stdlib;

use lexer::Lexer;
use parser::Parser;
use ovm::OVM;

/// CORE INITIALIZATION VECTOR
/// Phase 1.3: Sub-system Integration and Execution Engine
fn main() {
    println!("[KERNEL] Booting the Magick Programming Language (MPL) Framework...");
    println!("[KERNEL] Target Architecture: Apple M4 Max Unified Memory (Simulated)");
    
    // The raw esoteric intent vector (MagickScript)
    let source_code = r#"
        sacrifice btc_stop = 369.0
        sacrifice directive = "PREVENT LOSS"
        invoke hash_intent()
        invoke synchronize_mlx()
        transmute directive
    "#;

    println!("[COMPILER] Ingesting .ms payload...");
    
    // Phase 1: Tokenization
    println!("[LEXER] Collapsing textual noise into absolute tokens...");
    let lexer = Lexer::new(source_code);
    
    // Phase 2: AST Construction
    println!("[PARSER] Assembling Abstract Syntax Tree...");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("[PARSER] AST generated successfully. System ready for runtime transfer.");

    // Phase 3: Runtime Execution
    println!("[OVM] Initializing Occult Virtual Machine at 432.0 Hz baseline.");
    let mut ovm = OVM::new(432.0);
    
    println!("[EXECUTION] Resolving Intent and anchoring state variables...");
    ovm.execute(program);

    println!("[KERNEL] Execution complete. Noise obliterated. State safely anchored.");
}
