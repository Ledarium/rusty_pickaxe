use rand::Rng;
use rustc_hex::{FromHex, ToHex};
use std::time::Instant;
use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};
use serde::{Deserialize, Serialize};

#[cfg(feature = "cuda")]
#[link(name = "keccak", kind = "static")]
extern "C" {
    fn h_gpu_init() -> u32;
    fn h_set_block(data: *const u8);
    fn h_mine(data: *mut u8, start_nonce: u64, counts: u32, target: u64, block: u32, grid: u32) -> u64;
}

#[derive(Debug, Serialize)]
pub struct SecondBlock {
    pub eth_nonce: [u64; 4], //256
    pub salt: [u64; 4], //256
    pub pad_first: u8, // 8
    // wow rust sucks
    pub zero_pad0: [u64; 8], // 512
    pub zero_pad1: [u8; 6], // 48
    pub pad_last: u8, // 8
    // looks like total is 1088, which is what we need
}

impl SecondBlock {
    fn randomize_salt(&mut self) {
        self.salt[2] = rand::thread_rng().gen_range(0..u64::MAX);
    }
    fn get_real_salt(&self) -> u128 {
        u128::from(self.salt[3]) + u128::from(self.salt[2])*(u128::from(u64::MAX)+1)
    }
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
pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u128 {
    let work = serialize_work(&pre_work); // size is 168 bytes, 32 more is salt
    // split work into parts, first will be keccakFfed and stored in memory
    let first_block: [u8; 136] = work[0..136].try_into().expect("super bad");
    let first_block_hex: String = first_block.to_hex();
    debug!("first block: {}", first_block_hex);
    // second contains nonce and salt, needs to be padded and keccakFfed
    let mut second = SecondBlock { 
        eth_nonce: [0, 0, 0, pre_work.eth_nonce.into()],
        salt: [0; 4],
        pad_first: 0x01,
        zero_pad0: [0; 8],
        zero_pad1: [0; 6],
        pad_last: 0x80,
    };
    let mut hashes_done = 0u64;
    //let thr_id = 0;
    let mp_count = unsafe {h_gpu_init()};
    let cuda = CudaSettings { device_id: 0, block: mp_count, grid: 380, counts: 300 };
    debug!("GPU init");

    unsafe { h_set_block(first_block.as_ptr()) };
    debug!("Block set");

    let target = u64::from_be_bytes(target[0..8].try_into().expect("bad"));
    //debug!("Number of threads is {}", throughput);
    let mut start_nonce = 0u64;
    let mut res_salt = u64::MAX;
    let mut start_time = Instant::now();
    second.randomize_salt();
    let mut second_block_bytes = serialize_work(&second);
    let second_block_hex: String = second_block_bytes.to_hex();
    debug!("second block: {}", second_block_hex);
    let second_block_ptr = second_block_bytes.as_mut_ptr();
    while res_salt == u64::MAX {
        let ret = unsafe { h_mine(second_block_ptr, start_nonce, cuda.counts, target, cuda.block, cuda.grid)};
        if ret != u64::MAX { res_salt = ret; break; }
        start_nonce += cuda.throughput();
        if u64::MAX - hashes_done > cuda.throughput() {
            hashes_done += cuda.throughput();
        } else {
            info!("not found :( try another one!");
            second.randomize_salt();
            start_time = Instant::now();
            start_nonce = 0;
        }
        if start_nonce % 50000000 < cuda.throughput() {
            let elapsed = start_time.elapsed();
            println!("Elapsed time: {:.2?}, hashrate = {:.3?}MH/s", elapsed, hashes_done as f32/elapsed.as_secs_f32() / 1_000_000f32);
        }
    }
    second.salt[3] = res_salt;
    let mut second_block_bytes = serialize_work(&second);
    let second_block_hex: String = second_block_bytes.to_hex();
    debug!("second block: {}", second_block_hex);
    debug!("random number: {}", second.salt[2]);
    return second.get_real_salt();
}

#[cfg(not(feature = "cuda"))]
pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u128 {
    return 0;
}
