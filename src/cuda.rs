use rand::Rng;
use log::{debug, info};
use std::convert::TryInto;
use crate::utils::{PreWork, serialize_work, prepare_data};
use serde::{Deserialize, Serialize};

#[link(name = "keccak", kind = "static")]
extern "C" {
    fn h_gpu_init() -> u32;
    fn h_set_block(data: *const u8);
    fn h_mine(data: *const u8, end_nonce: u64, target: u64, block: u32, grid: u32) -> u64;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondBlock {
    _pad: [u64; 3],
    pub eth_nonce: u64,
    pub salt: [u64; 4],
}

impl SecondBlock {
    fn randomize_salt(&mut self) {
        self.salt[2] = rand::thread_rng().gen_range(0..u64::MAX);
    }
    fn get_real_salt(&self) -> u128 {
        u128::from(self.salt[3]) + u128::from(self.salt[2])*u128::from(u64::MAX)
    }
}

#[derive(Debug)]
struct CudaSettings {
    pub device_id: u32,
    pub block: u32,
    pub grid: u32,
}
impl CudaSettings {
    fn throughput(&self) -> u64 {
        (self.block * self.grid).into()
    }
}

pub fn mine_cuda(pre_work: &PreWork, target: [u8; 32]) -> u128 {
    let work = serialize_work(&pre_work); // size is 168 bytes, 32 more is salt
    // split work into parts, first will be keccakFfed and stored in memory
    let first_block: [u8; 136] = work[0..136].try_into().expect("super bad");
    // second contains nonce and salt, needs to be padded and keccakFfed
    let mut second = SecondBlock { eth_nonce: pre_work.eth_nonce.into(), salt: [0u64; 4], _pad: [0u64; 3] };
    let mut hashes_done = 0u64;
    //let thr_id = 0;
    let cuda = CudaSettings { device_id: 0, block: 1, grid: 2 };
    unsafe {h_gpu_init()};
    debug!("GPU init");

    unsafe { h_set_block(first_block.as_ptr()) };
    debug!("Block set");

    let target = u64::from_be_bytes(target[0..8].try_into().expect("bad"));
    //debug!("Number of threads is {}", throughput);
    let mut res_salt = u64::MAX;
    while res_salt == u64::MAX {
        second.salt[3] += cuda.throughput();
        debug!("Next to mine is {:?}, done {:?}", second.salt[3], hashes_done);
        let second_block_ptr = serialize_work(&second).as_ptr();
        let ret = unsafe { h_mine(second_block_ptr, second.salt[3]+cuda.throughput(), target, cuda.block, cuda.grid)};
        debug!("111111 ret={:?}", ret);
        if ret != u64::MAX { res_salt = ret; break; }
        debug!("222222");
        if u64::MAX - hashes_done > cuda.throughput() {
            debug!("333333");
            hashes_done += cuda.throughput();
        } else {
            debug!("not found :( try another one!");
        }
    }
    second.salt[0] = res_salt;
    return second.get_real_salt();
}
