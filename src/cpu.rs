use crate::utils::{serialize_work, Work};
use log::debug;
use tiny_keccak::{Hasher, Keccak};

pub fn prepare_data(work: &Work) -> Keccak {
    let mut h = Keccak::v256();
    h.update(&serialize_work(&work.first_block));
    //debug!("Keccak hex data: {}", String::from(bytes.to_hex()));
    h.update(&serialize_work(&work.second_block)[0..56]);
    return h;
}

pub fn optimized_hash(mut h: Keccak, salt: u64) -> [u8; 32] {
    h.update(&salt.to_be_bytes());
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

pub fn simple_hash(work: &Work) -> [u8; 32] {
    let mut h = Keccak::v256();
    h.update(&serialize_work(&work.first_block));
    h.update(&serialize_work(&work.second_block)[0..64]);
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

pub fn ez_cpu_mine(work: &Work) -> u64 {
    debug!("Got work {:?}", work);
    let keccak = prepare_data(work);
    let mut hash;
    let mut found = u64::MAX;
    for iter in work.start_nonce..work.end_nonce {
        //let salt = rand::thread_rng().gen_range(0..u128::MAX);
        let salt = iter;
        hash = optimized_hash(keccak.clone(), salt);
        //hash = simple_hash(work, salt);
        for index in 0..32 {
            //idk rusty way to write this
            if hash[index] > work.target[index] {
                break;
            } else if hash[index] < work.target[index] {
                found = salt;
                break;
            }
        }
        if found != u64::MAX {
            break;
        };
    }
    found
}

#[cfg(test)]
mod tests {
    use crate::cpu::{optimized_hash, prepare_data, simple_hash};
    use crate::utils::*;
    use rustc_hex::ToHex;
    use std::str::FromStr;
    use web3::types::Address;

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
        end_nonce: u64::MAX,
        target: [0xFE; 32],
    };
    #[test]
    fn test_seq_hash() {
        for i in 1u64..3u64 {
            let mut nonzero_work = ZERO_WORK.clone();
            nonzero_work.second_block.salt[3] = i;
            let owork = prepare_data(&nonzero_work);
            let shash: String = simple_hash(&nonzero_work).to_hex();
            let ohash: String = optimized_hash(owork, i).to_hex();
            assert_eq!(shash, ohash);
        }
    }

    #[test]
    fn test_zero_simple_hash() {
        let shash: String = simple_hash(&ZERO_WORK).to_hex();
        assert_eq!(
            shash,
            "e1bb54e1bc3af48d01e5dbfc81015c98152a574f6428c6948aa4837c9c0baad9"
        );
    }

    #[test]
    fn test_example_simple_hash() {
        let ex_work = Work {
            first_block: WorkFirstBlock {
                _pad1: [0u32; 7],
                chain_id: 1u32,
                entropy: [98u8; 32],
                _pad2: [0u32; 7],
                gem_address: Address::from_str("0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0")
                    .unwrap()
                    .to_fixed_bytes(),
                sender_address: Address::from_str("0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1")
                    .unwrap()
                    .to_fixed_bytes(),
                gem_id: 1u32,
            },
            second_block: WorkSecondBlock {
                contract_nonce: [0, 0, 0, 20],
                salt: [0, 0, 0, 2],
                pad_first: 0x01, // see keccak specifications for explaination
                zero_pad0: [0; 8],
                zero_pad1: [0; 6],
                pad_last: 0x80,
            },
            start_nonce: 0,
            end_nonce: u64::MAX,
            target: [0xFE; 32],
        };
        let shash: String = simple_hash(&ex_work).to_hex();
        assert_eq!(
            shash,
            "a569d9eb26b08c52dd21a023c8310550767a47c8a33035946ac25d404d7717ab"
        );
    }

    #[test]
    fn test_zero_optimized_hash() {
        let owork = prepare_data(&ZERO_WORK);
        let ohash: String = optimized_hash(owork, 0).to_hex();
        assert_eq!(
            ohash,
            "e1bb54e1bc3af48d01e5dbfc81015c98152a574f6428c6948aa4837c9c0baad9"
        );
    }

    #[test]
    fn test_keccak_clone() {
        // just to be sure lol
        use bincode::Options;
        use tiny_keccak::{Hasher, Keccak};
        let mut h0 = Keccak::v256();
        let bytes = serialize_work(&ZERO_WORK);
        h0.update(&bytes);
        h0.update(&[0u8, 16]); //salt high bits

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
