extern crate cc;

static CUDA_PATH: &str = "/opt/cuda/include";

fn main() {
  cc::Build::new()
    .cuda(true)
    .cpp(true)
    .flag("-cudart=shared")
    .files(&["./src/cuda_keccak256.cu", "./src/keccak256.c"])
    .compile("keccak256.a");
  println!("cargo:rustc-link-search=native={}", CUDA_PATH);
  println!("cargo:rustc-link-search={}", CUDA_PATH);
  println!("cargo:rustc-env=LD_LIBRARY_PATH={}", CUDA_PATH);
  println!("cargo:rustc-link-lib=dylib=cudart");
}
