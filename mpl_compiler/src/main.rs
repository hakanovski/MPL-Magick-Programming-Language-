mod lexer;
mod parser;
mod ast;
mod ovm;

use lexer::Lexer;
use parser::Parser;
use ovm::OccultVM;

/// CORE INITIALIZATION VECTOR
/// Phase 1: Occult Virtual Machine entry point.
fn main() {
    let source_code = r#"
        sacrifice btc_stop = 369.0
        sacrifice directive = "PREVENT LOSS"
        transmute directive
    "#;

    println!("--- INITIALIZING MAGICK PROGRAMMING LANGUAGE (MPL) ---");
    println!("Base Frequency Alignment: 432 Hz");
    println!("Ingesting MagickScript (.ms) vector...");

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("AST Generation Complete.\n");

    let mut ovm = OccultVM::new(432.0);
    ovm.execute(program);

    println!("--- EXECUTION OBLITERATED NOISE. STATE ANCHORED. ---");
}
