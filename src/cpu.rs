use tiny_keccak::{Hasher, Keccak};
use log::{debug, info};
use std::convert::TryInto;
use std::time::Instant;
use bincode::Options;
use rustc_hex::{FromHex, ToHex};
use crate::utils::PreWork;

pub struct OptimizedWork {
    pub keccak: Keccak,
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
        keccak: h.clone(),
        target: [0xFFu8; 32],
    };
    let target_bytes = target.to_be_bytes();
    for i in 0..16 {
        ret.target[i] = target_bytes[i]
    }
    return ret;
}

pub fn optimized_hash(work: &OptimizedWork, salt: u128) -> [u8; 32] {
    let mut h = work.keccak.clone();
    //debug!("Before {:?}", h.state.buffer);
    h.update(&salt.to_be_bytes());
    //debug!("After  {:?}", h.state.buffer);
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

pub fn simple_hash(pre_work: &PreWork, salt: u128) -> [u8; 32] {
    let mut h = Keccak::v256();
    let bytes = bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();
    h.update(&bytes);
    h.update(&[0u8; 16]);
    h.update(&salt.to_be_bytes());
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

pub fn ez_cpu_mine (pre_work: &PreWork, target: u128) -> u128 {
    let owork = prepare_data(pre_work, target);
    info!("Starting mining, target {:?}", u128::from_be_bytes(owork.target[16..32].try_into().unwrap()));
    let start_time = Instant::now();
    let mut hash = [0u8;32];
    let mut found = 0u128;
    for iter in 0..u128::MAX {
        //let salt = rand::thread_rng().gen_range(0..u128::MAX);
        let salt = iter;
        /*
        hash = optimized_hash(&owork, salt);
        */
        hash = simple_hash(pre_work, salt);
        for index in 0..32 { //idk rusty way to write this
            if hash[index] > owork.target[index] {
                break;
            } else if hash[index] < owork.target[index] {
                found = salt;
                break;
            }
        }
        if iter % 500000 == 1 {
            debug!("Trying salt {}", salt);
            let elapsed = start_time.elapsed();
            println!("Elapsed time: {:.2?}, hashrate = {}", elapsed, iter as f32/elapsed.as_secs_f32());
        }
        if found > 0 { break };
    }
    let string_hash: String = hash.to_hex();
    let string_target: String = owork.target.to_hex();
    debug!("Hash:   {}", string_hash);
    debug!("Target: {}", string_target);
    return found;
}

#[cfg(test)]
mod tests {
    use rustc_hex::ToHex;
    use crate::utils::PreWork;
    use crate::cpu::{prepare_data, optimized_hash, simple_hash};

    #[test]
    fn test_seq_hash() {
        let pre_work = PreWork {
            _pad1: [0u32; 7],
            chain_id: 0u32,
            entropy: [0u8; 32],
            _pad2: [0u32; 7],
            gem_id: 0u32,
            gem_address: [0u8; 20],
            sender_address: [0u8; 20],
            _pad3: [0u32; 7],
            eth_nonce: 0u32,
        };
        for i in 1..3 {
            let owork = prepare_data(&pre_work, 0);
            let shash: String = simple_hash(&pre_work, i).to_hex();
            let ohash: String = optimized_hash(&owork, i).to_hex();
            println!("shash {}\nohash {}",shash, ohash);
        }
    }

    #[test]
    fn test_optimized_hash() {
        let pre_work = PreWork {
            _pad1: [0u32; 7],
            chain_id: 0u32,
            entropy: [0u8; 32],
            _pad2: [0u32; 7],
            gem_id: 0u32,
            gem_address: [0u8; 20],
            sender_address: [0u8; 20],
            _pad3: [0u32; 7],
            eth_nonce: 0u32,
        };
        let owork = prepare_data(&pre_work, 0);
        let shash: String = simple_hash(&pre_work, 0).to_hex();
        let ohash: String = optimized_hash(&owork, 0).to_hex();
        assert_eq!(shash, ohash);
    }

}
