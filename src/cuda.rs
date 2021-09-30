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
    let cuda = CudaSettings { device_id: 0, block: mp_count, grid: 256, counts: 300 };
    debug!("GPU init");

    let mut first_block_bytes = serialize_work(&work.first_block);
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
    debug!("cuda res_salt={}", res_salt);
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
pub fn mine_cuda(work: &Work) -> u64 {
    return u64::MAX;
}

#[cfg(feature = "cuda")]
#[cfg(test)]
mod tests {
    use rustc_hex::ToHex;
    use crate::utils::*;
    use crate::cpu::simple_hash;
    use crate::cuda::mine_cuda;
    use web3::types::Address;
    use std::str::FromStr;
    use bigint::uint::U256 as u256;
    use log::{debug, info};

    static ZERO_WORK: Work = Work {
        first_block: WorkFirstBlock {
            _pad1: [0u32; 7],
            chain_id: 0u32,
            entropy: [0u8; 32],
            _pad2: [0u32; 7],
            gem_id: 0u32,
            gem_address: [0u8; 20],
            sender_address: [0u8; 20],
        },
        second_block: WorkSecondBlock {
            contract_nonce: [0, 0, 0, 0],
            salt: [0; 4],
            pad_first: 0x01, 
            pad_last: 0x80, // see keccak specifications for explaination
            zero_pad0: [0; 8],
            zero_pad1: [0; 6],
        }, 
        start_nonce: 0,
        end_nonce: 10,
        target: [0x11; 32]
    };

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_seq_hash() {
        init();
        for i in 0x2222u64..0x2223u64 {
            let mut nonzero_work = ZERO_WORK.clone();
            nonzero_work.second_block.salt[2] = i;
            let cuda_salt = mine_cuda(&nonzero_work);
            nonzero_work.second_block.salt[3] = cuda_salt;
            let cpu_hash = &simple_hash(&nonzero_work);
            let cpu_hash_hex: String = cpu_hash.to_hex();
            debug!("Cpu hash is {}", cpu_hash_hex);
            assert!(u256::from_big_endian(cpu_hash) < u256::from_big_endian(&nonzero_work.target));
        }
    }
}
