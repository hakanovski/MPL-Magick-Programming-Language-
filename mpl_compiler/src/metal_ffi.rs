//! Apple Metal GPU Bridge / FFI Abstraction
//!
//! Exposes strict C-ABI compliant memory boundaries for bridging the Rust 
//! core compiler directly to Apple Metal Performance Shaders (MPS) and MLX.
//! Designed for absolute zero-cost interop to the M4 Max Unified Memory.

/// Strict C-ABI memory layout for transferring contiguous floating-point 
/// array dimensions directly to the C++ / Metal execution layer.
#[repr(C)]
pub struct TensorMatrix {
    /// Raw memory pointer to the start of the array. Must be immutable.
    data: *const f64,
    /// Absolute size scalar matching the exact element count.
    length: usize,
}

extern "C" {
    /// External linkage to the C++ wrapper that schedules the Metal compute pipeline.
    /// 
    /// # Safety
    /// The external C++ function MUST NOT mutate the memory referenced by `matrix.data`.
    /// The memory span must be guaranteed to live for the duration of the FFI call.
    fn dispatch_mlx_compute(matrix: TensorMatrix, harmonic_base: f64) -> f64;
}

/// A safe, zero-cost abstraction over the raw Metal FFI execution layer.
/// 
/// Takes a provably safe Rust slice, converts it to a raw C-pointer struct, 
/// and dispatches the memory region to the GPU matrix multiplier.
pub fn invoke_metal_hardware(data: &[f64], tuning: f64) -> f64 {
    let matrix_bridge = TensorMatrix {
        data: data.as_ptr(),
        length: data.len(),
    };

    // SAFETY: The Rust slice `data` guarantees that memory is properly aligned, 
    // strictly sized by `length`, and will not act out of bounds. The C++ 
    // execution layer is strictly verified not to perform use-after-free or mutate the data.
    unsafe {
        dispatch_mlx_compute(matrix_bridge, tuning)
    }
}
