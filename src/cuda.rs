use rustc_hex::{FromHex, ToHex};
use std::time::Instant;
use log::{debug, info};
use std::convert::TryInto;
use crate::utils::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "cuda")]
#[link(name = "keccak", kind = "static")]
extern "C" {
    fn h_gpu_init() -> u32;
    fn h_set_block(data: *const u8);
    fn h_mine(data: *mut u8, start_nonce: u64, counts: u32, target: u64, block: u32, grid: u32) -> u64;
}

#[derive(Debug)]
struct CudaSettings {
    pub device_id: u32,
    pub block: u32,
    pub grid: u32,
    pub counts: u32,
}
impl CudaSettings {
    fn throughput(&self) -> u64 {
        (self.counts * self.block * self.grid).into()
    }
}

#[cfg(feature = "cuda")]
pub fn mine_cuda(work: &Work) -> u64 {
    //let thr_id = 0;
    let mp_count = unsafe {h_gpu_init()};
    let cuda = CudaSettings { device_id: 0, block: mp_count, grid: 380, counts: 300 };
    debug!("GPU init");

    let mut first_block_bytes = serialize_work(&work.second_block);
    unsafe { h_set_block(first_block_bytes.as_ptr()) };
    debug!("Block set");

    let target = u64::from_be_bytes(work.target[0..8].try_into().expect("bad"));
    //debug!("Number of threads is {}", throughput);
    let mut res_salt = u64::MAX;
    let mut second_block_bytes = serialize_work(&work.second_block);
    let second_block_ptr = second_block_bytes.as_mut_ptr();
    let mut start_nonce = work.start_nonce;
    while start_nonce < work.end_nonce {
        let ret = unsafe { h_mine(second_block_ptr, work.start_nonce, cuda.counts, target, cuda.block, cuda.grid)};
        if ret != u64::MAX { res_salt = ret; break; }
        start_nonce += cuda.throughput();
    }
    res_salt
    /*
    let mut second_block_bytes = serialize_work(&second);
    let second_block_hex: String = second_block_bytes.to_hex();
    debug!("second block: {}", second_block_hex);
    debug!("random number: {}", second.salt[2]);
    second.get_real_salt()
    */
}

#[cfg(not(feature = "cuda"))]
pub fn mine_cuda(work: Work) -> u64 {
    return u64::MAX;
}
