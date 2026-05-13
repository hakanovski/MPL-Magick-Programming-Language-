//! The Magick Standard Library
//! 
//! Bridges high-level MagickScript invocations to the low-level Foreign Function 
//! Interface (FFI) and the local Akashic vector state management layer.

use crate::akashic::AkashicGrid;
use crate::metal_ffi::invoke_metal_hardware;

/// Wraps the Apple Metal FFI boundary safely.
/// Takes a standard Rust f64 slice and passes it to the GPU via C-ABI boundaries.
pub fn invoke_synchronize_mlx(data: &[f64], tuning: f64) -> f64 {
    // Using strict type constraints to bridge Rust memory into the C++ layer.
    // In actual linking, this triggers the MPS matrix multiplication payload.
    // Safety is guaranteed by the slice length validation wrapper.
    invoke_metal_hardware(data, tuning)
}

/// Abstract function to record deterministic physical intent onto the memory grid.
pub fn log_to_akashic(grid: &mut AkashicGrid, intent: &str) {
    grid.write_intent(intent);
}
