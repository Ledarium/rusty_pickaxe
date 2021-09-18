use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};

#[link(name = "cuda_keccak256", kind = "static")]
extern "C" {
    fn keccak256_setBlock_80(endiandata: *mut u64);
    fn prepare_mining(thr_id: u32, throughput: u32, data: *const u64, targetH: u32, targetL: u32) -> u32;
    fn keccak256_cpu_hash_80(thr_id: u32, throughput: u32, first_nonce: u32, nonces: *mut u32) -> u32;
}

pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u32 {
    let mut hashes_done = 0;
    let mut restart = false;
    let mut work = serialize_work(&pre_work);
    let throughput = u32::MAX / 2;
    let thr_id = 0;
    let keccak = prepare_data(&pre_work);
    debug!("preparing");
    let prepare_rc = unsafe { prepare_mining(
            thr_id,
            throughput,
            keccak.state.buffer.0.as_ptr(),
            u32::from_be_bytes(target[24..28].try_into().expect("bad")),
            u32::from_be_bytes(target[28..32].try_into().expect("bad")),
        ) };
    debug!("prepare returns {}, mining", prepare_rc);
    let mut nonces: [u32; 2] = {u32::MAX; u32::MAX};
    let result = unsafe { mining_iter(thr_id, throughput, 0, nonces.to_mut_ptr()) };
    return result;
}
