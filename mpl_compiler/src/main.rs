mod lexer;
mod parser;
mod ast;
mod ovm;
mod gematria;
mod harmonic;
mod akashic;
mod metal_ffi;
mod stdlib;
mod oracle;
mod grimoire;
mod aether;
mod execution;
mod gateway;
mod mlx_engine;
mod sigil;
mod evolution;
mod entropy;

use lexer::Lexer;
use parser::Parser;
use ovm::OVM;
use oracle::Oracle;
use aether::{AetherSource, KrakenProvider};
use execution::MarketExecutor;
use std::env;
use std::thread;
use std::time::Duration;

/// CORE INITIALIZATION VECTOR
/// Phase 3.1: The Grimoire I/O & The Aetheric Network
#[tokio::main]
async fn main() {
    println!("[KERNEL] Booting the Magick Programming Language (MPL) Daemon...");
    println!("[KERNEL] Target Architecture: Apple M4 Max Unified Memory (Simulated)");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("[KERNEL_PANIC] Insufficient launch parameters. Formats:");
        eprintln!("  cargo run -- execute <path_to_script.ms>");
        eprintln!("  cargo run -- daemon");
        return;
    }

    let mode = &args[1];

    match mode.as_str() {
        "execute" => {
            if args.len() < 3 {
                eprintln!("[KERNEL_PANIC] Missing manuscript path. Usage: execute <path>");
                return;
            }
            let file_path = &args[2];
            println!("[GRIMOIRE] Ingesting physical scroll: {}", file_path);
            
            match grimoire::load_script(file_path) {
                Ok(source_code) => {
                    execute_script(&source_code);
                }
                Err(e) => {
                    eprintln!("[KERNEL_FAULT] Failed to ingest manuscript: {}", e);
                }
            }
        }
        "daemon" => {
            println!("[KERNEL] Initializing Daemon Oracle loop and Altar Gateway...");
            let daemon_task = tokio::spawn(run_daemon());
            let gateway_task = tokio::spawn(gateway::start_gateway());
            
            let _ = tokio::join!(daemon_task, gateway_task);
        }
        _ => {
            eprintln!("[KERNEL_PANIC] Unknown operational mode: {}", mode);
        }
    }
}

/// Helper function to cleanly parse and execute a MagickScript payload.
fn execute_script(source_code: &str) {
    println!("[LEXER] Collapsing textual noise into absolute tokens...");
    let lexer = Lexer::new(source_code);
    
    println!("[PARSER] Assembling Abstract Syntax Tree...");
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    println!("[OVM] Initializing Occult Virtual Machine at 432.0 Hz baseline.");
    let mut ovm = OVM::new(432.0, Box::new(MarketExecutor::new()));
    
    println!("[EXECUTION] Resolving Intent and anchoring state variables...");
    ovm.execute(program);

    println!("[KERNEL] Execution complete. State safely anchored.");
}

/// The continuous simulation loop bridging the Oracle to the Aether network.
async fn run_daemon() {
    let oracle = Oracle::new();
    let mut ovm = OVM::new(432.0, Box::new(MarketExecutor::new()));
    let aether = KrakenProvider::new();

    println!("[DAEMON] Telemetry Oracle online. Neural Link Active. Synchronizing with the live Aether...");

    // Tearing down the simulation. Buffer for aggregating live ticks before OVM ingestion.
    let mut tick_buffer: Vec<f64> = Vec::with_capacity(50);
    let target_pair = "XBT/USD";

    loop {
        println!("\n[DAEMON_HEARTBEAT] Engaging live WebSocket hook for {}...", target_pair);

        let stream_result = aether.stream_ticks(target_pair, Box::new(|live_price| {
            tick_buffer.push(live_price);

            // Flush to the oracle when saturation is reached
            if tick_buffer.len() >= 50 {
                // Pass the raw memory slice directly to the Oracle
                if let Some(anomalies) = oracle.ingest_tick_stream(&tick_buffer) {
                    println!("[ORACLE] ⚠️ HARMONIC ANOMALY DETECTED. Vector Count: {}", anomalies.len());
                    println!("[ORACLE] Engaging Occult Virtual Machine (OVM) interception protocol for anomalies...");

                    // Dynamic script generation based on oracle detection
                    let dynamic_intent = format!(
                        r#"
                        sacrifice anomaly_threshold = 369.0
                        sacrifice directive = "ANOMALY RESOLVED [{}]"
                        invoke hash_intent()
                        invoke synchronize_mlx()
                        transmute directive
                        "#,
                        anomalies.len()
                    );

                    // JIT Lex & Parse the dynamic intent
                    let lexer = Lexer::new(&dynamic_intent);
                    let mut parser = Parser::new(lexer);
                    let program = parser.parse_program();

                    // Execute the deterministic response
                    ovm.execute(program);
                    
                    println!("[KERNEL] OVM Execution complete. Thermodynamic state secured.");
                }

                // Clear the buffer for the next temporal slice, retaining pre-allocated capacity
                tick_buffer.clear();
            }
        })).await;

        if let Err(e) = stream_result {
            eprintln!("[AETHER_FAULT] Live stream fracture detected: {}", e);
            println!("[DAEMON] Initiating temporal backoff to prevent network flooding...");
            // Non-blocking sleep for the async loop
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
