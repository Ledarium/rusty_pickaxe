use tiny_keccak::{Hasher, Keccak};
use log::{debug, info};
use std::convert::TryInto;
use std::time::Instant;
//use crate::utils::PreWork;

pub struct OptimizedWork {
    pub data: Vec<u8>,
    pub salt_high: u128,
    pub salt_low: u128,
    pub target: [u8; 32],
}

/*
pub fn prepare_data(pre_work: &PreWork) -> OptimizedWork {
    let mut h = Keccak::v256();
    h.update(&pre_work.data);
    h.update(&pre_work.salt_high.to_be_bytes());
*/


pub fn optimized_hash(work: &OptimizedWork) -> [u8; 32] {
    let mut h = Keccak::v256();
    h.update(&work.data);
    h.update(&work.salt_high.to_be_bytes());
    h.update(&work.salt_low.to_be_bytes());
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

pub fn ez_cpu_mine (owork: &mut OptimizedWork) -> u128 {
    info!("Starting mining, target {:?}", u128::from_be_bytes(owork.target[16..32].try_into().unwrap()));

    let start_time = Instant::now();
    let mut hash = [0u8; 32];
    let mut found = 0u128;
    for salt in 0..u128::MAX {
        if salt % 500000 == 1 {
            debug!("Trying salt {}", salt);
            let elapsed = start_time.elapsed();
            println!("Elapsed time: {:.2?}, hashrate = {}", elapsed, salt as f32/elapsed.as_secs_f32());
        }
        owork.salt_low = salt;
        hash = optimized_hash(&owork);
        for index in 0..32 { //idk rusty way to write this
            if hash[index] < owork.target[index] {
                break;
            } else if index == 32 {
                found = salt;
            }
        }
    }
    return found;
}
