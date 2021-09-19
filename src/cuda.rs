use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};

#[link(name = "cuda_keccak256", kind = "static")]
extern "C" {
    fn gpu_init();
    fn h_set_block(data: *const u8); //136 bytes
    fn h_mine(data: *const u8, start_nonce: u32, target: u64, res_nonce: *mut u32) -> u32;
}

pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u32 {
    let mut hashes_done = 0;
    let mut work = serialize_work(&pre_work);
    let throughput = 4096*100;
    let thr_id = 0;
    unsafe {gpu_init()};

    let target_high = u64::from_be_bytes(target[0..8].try_into().expect("bad"));
    unsafe { h_set_block(0, target) };
    debug!("prepare returns {}, mining, target_high {}, target_low {}", prepare_rc, target_high, target_low);
    let mut nonces: [u32; 2] = [u32::MAX, u32::MAX];
    let mut result = u32::MAX;
    let mut start_nonce = 0;
    while result == u32::MAX {
        unsafe { keccak256_cpu_hash_80(thr_id, throughput, start_nonce, nonces.as_mut_ptr()) };
        if nonces[0] != u32::MAX { return nonces[0]; }
        else if nonces[1] != u32::MAX { return nonces[1]; }
        nonces = [u32::MAX, u32::MAX];
        hashes_done += throughput;
        start_nonce += throughput;
        debug!("Next to mine is {:?}, done {:?}", start_nonce, hashes_done);
    }
    return u32::MAX;
}
