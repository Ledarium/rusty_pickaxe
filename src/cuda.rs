use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};

#[link(name = "cuda_keccak256", kind = "static")]
extern "C" {
    fn h_gpu_init() -> u32;
    fn h_set_block(data: *const u8); //136 bytes
    fn h_mine(data: *const u8, start_nonce: u32, target: u64) -> u32;
}

pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u32 {
    let work = serialize_work(&pre_work); // size is 168 bytes, 32 more is salt
    // split work into parts, first will be keccakFfed and stored in memory
    // second contains nonce and salt, needs to be padded and keccakFfed
    let first_block: [u8; 136] = work[0..136].try_into().expect("super bad");
    let mut second_block: [u8; 64] = [0; 64];
    for i in 0..32 {
        second_block[i] = work[136+i] // TODO: a bit dirty, rewrite this
    }
    let mut hashes_done = 0u32;
    //let thr_id = 0;
    debug!("Uhmmm..");
    let throughput = unsafe {h_gpu_init()};
    debug!("Number of threads is {}", throughput);

    debug!("GPU init");
    let target = u64::from_be_bytes(target[0..8].try_into().expect("bad"));
    unsafe { h_set_block(first_block.as_ptr()) };
    debug!("Block set");
    let mut res_nonce = u32::MAX;
    let mut start_nonce = 0;
    while res_nonce == u32::MAX {
        let ret = unsafe { h_mine(second_block.as_ptr(), start_nonce, target) };
        if ret != u32::MAX { res_nonce = ret; break; }
        if u32::MAX - hashes_done < throughput {
            hashes_done += throughput;
        } else {
            debug!("not found :(");
            return u32::MAX;
        }
        start_nonce += throughput;
        debug!("Next to mine is {:?}, done {:?}", start_nonce, hashes_done);
    }
    return res_nonce;
}
