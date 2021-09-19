extern crate cc;

static CUDA_PATH: &str = "/opt/cuda/include";

fn main() {
  cc::Build::new()
    .cuda(true)
    .flag("-cudart=shared")
    .flag("-gencode")
    .flag("arch=compute_86,code=sm_86") // RTX 30**
    .include("./src")
    .files(&["./src/keccak.cu"])
    .compile("libkeccak.a");
  println!("cargo:rustc-link-search=all={}", CUDA_PATH);
  println!("cargo:rustc-link-search=all={}", "./src");
  println!("cargo:rustc-env=LD_LIBRARY_PATH={}", CUDA_PATH);
  println!("cargo:rustc-link-lib=dylib=cudart");
}
