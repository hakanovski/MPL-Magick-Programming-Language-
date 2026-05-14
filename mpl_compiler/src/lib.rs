//! The Vessel (WASM Abstraction)
//! Exposes the OVM, Lexer, and Sigil Engine to the Web via WebAssembly.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ovm;
pub mod execution;
pub mod gematria;
pub mod sigil;
pub mod stdlib;
pub mod akashic;
pub mod benchmark;
pub mod entropy;
pub mod evolution;
pub mod metal_ffi;
pub mod mlx_engine;
pub mod gateway;
pub mod mesh;
pub mod signature;
pub mod parallel;
pub mod graph;
pub mod sonic;
pub mod chronos;
pub mod probability;
pub mod ledger;
pub mod sdk_bridge;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmOVM {
    inner: ovm::OVM,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmOVM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Use AppManifestExecutor for frontend abstraction
        let executor = Box::new(execution::AppManifestExecutor::new());
        Self {
            inner: ovm::OVM::new(432.0, executor, "WASM_FRONTEND_INTERFACE"),
        }
    }

    pub fn execute_script(&mut self, script: &str) -> String {
        let lexer = lexer::Lexer::new(script);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        self.inner.execute(program);

        // Serialize sigil data if present
        if let Some(ref sigil) = self.inner.last_visual_sigil {
            match serde_json::to_string(sigil) {
                Ok(json) => json,
                Err(_) => "[]".to_string(),
            }
        } else {
            "[]".to_string()
        }
    }
}
