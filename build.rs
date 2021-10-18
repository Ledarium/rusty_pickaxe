#[cfg(feature = "cuda")]
extern crate cc;

#[cfg(feature = "cuda")]
static CUDA_PATH: &str = "/opt/cuda/include";

#[cfg(feature = "cuda")]
fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
    cc::Build::new()
        .cuda(true)
        .flag("-cudart=shared")
        .flag("-gencode")
        .flag("arch=compute_50,code=sm_50") // GTX 7**
        .flag("-gencode")
        .flag("arch=compute_52,code=sm_52") // GTX 9**
        .flag("-gencode")
        .flag("arch=compute_60,code=sm_60") // P100
        .flag("-gencode")
        .flag("arch=compute_61,code=sm_61") // GTX 10**
        .flag("-gencode")
        .flag("arch=compute_75,code=sm_75") // RTX 20**, 1660 Ti
        .flag("-gencode")
        .flag("arch=compute_86,code=sm_86") // RTX 30**
        .include("./src")
        .files(&["./src/keccak.cu"])
        .compile("libkeccak.a");
    println!("cargo:rustc-link-search=all={}", CUDA_PATH);
    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", CUDA_PATH);
    println!("cargo:rustc-link-lib=dylib=cudart");
}

#[cfg(not(feature = "cuda"))]
fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
}
