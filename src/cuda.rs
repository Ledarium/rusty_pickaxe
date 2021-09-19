use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};
use serde::{Deserialize, Serialize};

#[link(name = "keccak", kind = "static")]
extern "C" {
    fn h_gpu_init() -> u32;
    fn h_set_block(data: *const u8); //136 bytes
    fn h_mine(data: *const u8, end_nonce: u32, target: u64, block: u32, grid: u32) -> u32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondBlock {
    _pad: [u32; 7],
    pub eth_nonce: u32,
    pub salt: [u32; 8],
}

pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u32 {
    let work = serialize_work(&pre_work); // size is 168 bytes, 32 more is salt
    // split work into parts, first will be keccakFfed and stored in memory
    let first_block: [u8; 136] = work[0..136].try_into().expect("super bad");
    // second contains nonce and salt, needs to be padded and keccakFfed
    let mut second = SecondBlock { eth_nonce: pre_work.eth_nonce, salt: [0u32; 8], _pad: [0u32; 7] };
    let mut hashes_done = 0u32;
    //let thr_id = 0;
    unsafe {h_gpu_init()};
    debug!("GPU init");

    unsafe { h_set_block(first_block.as_ptr()) };
    debug!("Block set");

    let target = u64::from_be_bytes(target[0..8].try_into().expect("bad"));
    //debug!("Number of threads is {}", throughput);
    let mut res_nonce = u32::MAX;
    let throughput = 4096;
    while (res_nonce == u32::MAX) && (hashes_done - throughput < u32::MAX){
        //let random: u64 = rand::thread_rng().gen_range(0..u64::MAX);
        second.salt[0] += hashes_done;
        debug!("Next to mine is {:?}, done {:?}", second.salt[0], hashes_done);
        let second_block_ptr = serialize_work(&second).as_ptr();
        let ret = unsafe { h_mine(second_block_ptr, second.salt[0]+throughput, target, 46, 256)};
        if ret != u32::MAX { res_nonce = ret; break; }
        if u32::MAX - hashes_done < throughput {
            hashes_done += throughput;
        } else {
            debug!("not found :(");
            return u32::MAX;
        }
    }
    return res_nonce;
}
