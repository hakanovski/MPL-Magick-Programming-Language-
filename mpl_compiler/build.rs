// The Native Compiler Hook
// Orchestrates the C++ compilation and Metal framework linkage
// for the esoteric MPL Virtual Machine.

fn main() {
    println!("cargo:rerun-if-changed=src/metal_kernel.cpp");

    let mut build = cc::Build::new();
    
    build.cpp(true)
         .file("src/metal_kernel.cpp")
         .flag("-std=c++17");

    // Gracefully handle cross-compiling or building on disparate architectures
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
    }

    build.compile("metal_kernel");
}
