use tiny_keccak::{Hasher, Keccak};
use log::{debug, info};
use std::convert::TryInto;
use std::time::Instant;
use bincode::Options;
use rustc_hex::{FromHex, ToHex};
use crate::utils::PreWork;

#[derive(Clone)]
pub struct OptimizedWork {
    pub keccak: Keccak,
    pub target: [u8; 32],
}

pub fn prepare_data(pre_work: &PreWork, target: &[u8; 32]) -> OptimizedWork {
    let mut h = Keccak::v256();
    let bytes = bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();
    h.update(&bytes);
    h.update(&[0u8,16]); //salt high bits
    let string: String = bytes.to_hex();
    debug!("hex data: {}", string);
    let mut ret = OptimizedWork {
        keccak: h.clone(),
        target: [0xFFu8; 32],
    };
    for i in 0..32 {
        ret.target[i] = target[i]
    }
    return ret;
}

pub fn optimized_hash(work: &OptimizedWork, salt: u128) -> [u8; 32] {
    let mut h = work.keccak.clone();
    h.update(&salt.to_be_bytes());
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

pub fn ez_cpu_mine (pre_work: &PreWork, target: [u8; 32]) -> u128 {
    let owork = prepare_data(pre_work, &target);
    let start_time = Instant::now();
    let mut hash = [0u8;32];
    let mut found = 0u128;
    for iter in 0..u128::MAX {
        //let salt = rand::thread_rng().gen_range(0..u128::MAX);
        let salt = iter;
        //hash = optimized_hash(&owork, salt);
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
    use web3::types::Address;
    use std::str::FromStr;

    static zero_work: PreWork = PreWork {
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
    #[test]
    fn test_seq_hash() {
        for i in 1..3 {
            let owork = prepare_data(&zero_work, &[0u8; 32]);
            let shash: String = simple_hash(&zero_work, i).to_hex();
            let ohash: String = optimized_hash(&owork, i).to_hex();
            assert_eq!(shash, ohash);
        }
    }

    #[test]
    fn test_zero_simple_hash() {
        let shash: String = simple_hash(&zero_work, 0).to_hex();
        assert_eq!(shash, "e1bb54e1bc3af48d01e5dbfc81015c98152a574f6428c6948aa4837c9c0baad9");
    }

    #[test]
    fn test_example_simple_hash() {
        let ex_work: PreWork = PreWork {
            _pad1: [0u32; 7],
            chain_id: 1u32,
            entropy: [98u8; 32],
            _pad2: [0u32; 7],
            gem_id: 1u32,
            gem_address: Address::from_str("0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0").unwrap().to_fixed_bytes(),
            sender_address: Address::from_str("0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1").unwrap().to_fixed_bytes(),
            _pad3: [0u32; 7],
            eth_nonce: 20u32,
        };
        let shash: String = simple_hash(&ex_work, 2).to_hex();
        assert_eq!(shash, "a569d9eb26b08c52dd21a023c8310550767a47c8a33035946ac25d404d7717ab");
    }

    #[test]
    fn test_zero_optimized_hash() {
        let owork = prepare_data(&zero_work, &[0u8; 32]);
        let ohash: String = optimized_hash(&owork, 0).to_hex();
        assert_eq!(ohash, "e1bb54e1bc3af48d01e5dbfc81015c98152a574f6428c6948aa4837c9c0baad9");
    }

    #[test]
    fn test_keccak_clone() {
        use bincode::Options;
        use tiny_keccak::{Hasher, Keccak};
        let mut h0 = Keccak::v256();
        let bytes = bincode::options()
            .with_fixint_encoding()
            .allow_trailing_bytes()
            .with_big_endian()
            .serialize(&zero_work).unwrap();
        h0.update(&bytes);
        h0.update(&[0u8,16]); //salt high bits

        let mut h1 = h0.clone();
        h1.update(&0u128.to_be_bytes());
        let mut res1 = [0u8; 32];
        h1.finalize(&mut res1);

        h0.update(&0u128.to_be_bytes());
        let mut res0 = [0u8; 32];
        h0.finalize(&mut res0);

        let s0: String = res0.to_hex();
        let s1: String = res1.to_hex();
        assert_eq!(s0, s1)
    }
}
