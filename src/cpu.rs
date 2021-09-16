use tiny_keccak::{Hasher, Keccak};
use log::{debug, info};
use std::convert::TryInto;
use std::time::Instant;
use bincode::Options;
use crate::utils::PreWork;

pub struct OptimizedWork {
    pub keccak: Keccak,
    pub salt_low: u128,
    pub target: [u8; 32],
}

pub fn prepare_data(pre_work: &PreWork, target: u128) -> OptimizedWork {
    let mut h = Keccak::v256();
    let bytes = bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();
    h.update(&bytes);
    h.update(&[0u8,16]); //salt high bits
    let mut ret = OptimizedWork {
        keccak: h,
        salt_low: 0,
        target: [0u8; 32],
    };
    let target_bytes = target.to_be_bytes();
    for i in 16..32 {
        ret.target[i] = target_bytes[i-16]
    }
    return ret;
}


pub fn optimized_hash(work: &OptimizedWork) -> [u8; 32] {
    let mut h = work.keccak.clone();
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
