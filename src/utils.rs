use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use web3::types::Address;
use bincode::Options;

pub fn serialize_work<T> (pre_work: &T) -> Vec<u8> where T:Serialize {
    return bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();
}

#[derive(Clone, Debug, Serialize)]
pub struct WorkSecondBlock {
    pub contract_nonce: [u64; 4], //256
    pub salt: [u64; 4], //256
    pub pad_first: u8, // 8
    // wow rust sucks
    pub zero_pad0: [u64; 8], // 512
    pub zero_pad1: [u8; 6], // 48
    pub pad_last: u8, // 8
    // looks like total is 1088, which is what we need
}
impl WorkSecondBlock {
    pub fn randomize_salt(&mut self) {
        self.salt[2] = rand::thread_rng().gen_range(0..u64::MAX);
    }
    pub fn get_real_salt(&self) -> u128 {
        u128::from(self.salt[3]) + u128::from(self.salt[2])*(u128::from(u64::MAX)+1)
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct WorkFirstBlock {
    pub _pad1: [u32; 7],
    pub chain_id: u32,
    pub entropy: [u8; 32],
    pub gem_address: [u8; 20],
    pub sender_address: [u8; 20],
    pub _pad2: [u32; 7],
    pub gem_id: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct Work {
    pub first_block: WorkFirstBlock,
    pub second_block: WorkSecondBlock,
    pub target: [u8; 32],
    pub start_nonce: u64,
    pub end_nonce: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Network {
    pub chain_id: String, //not used, getting from contract
    pub rpc: String,
    pub explorer: String, //not used, not there yet
    pub gem_address: Address,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Claim {
    pub private_key: String,
    pub maximum_gas_price: u32, //not used, not there yet
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub r#loop: bool,
    pub network: Network,
    pub gem_type: u32,
    pub address: Address,
    pub claim: Claim,
    pub cuda: bool,
    pub threads: usize,
}

pub fn vtoa<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[cfg(test)]
mod tests {
    use bincode::Options;
    use crate::utils::*;
    use web3::types::Address;
    use std::str::FromStr;
    use rustc_hex::ToHex;

    #[test]
    fn test_serialzing_first() {
        let ex_first_block = WorkFirstBlock {
            _pad1: [0u32; 7],
            chain_id: 1u32,
            entropy: [98u8; 32],
            _pad2: [0u32; 7],
            gem_address: Address::from_str("0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0").unwrap().to_fixed_bytes(),
            sender_address: Address::from_str("0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1").unwrap().to_fixed_bytes(),
            gem_id: 1u32,
        };
        let bytes: String = serialize_work(&ex_first_block).to_hex();
        assert_eq!(bytes, "00000000000000000000000000000000000000000000000000000000000000016262626262626262626262626262626262626262626262626262626262626262ffcf8fdee72ac11b5c542428b35eef5769c409f090f8bf6a479f320ead074411a4b0e7944ea8c9c10000000000000000000000000000000000000000000000000000000000000001");
    }
    #[test]
    fn test_serialzing_last() {
        let mut second_block = WorkSecondBlock {
            contract_nonce: [1, 2, 3, 4],
            salt: [u64::MAX; 4],
            pad_first: 0x01, // see keccak specifications for explaination
            zero_pad0: [0; 8],
            zero_pad1: [0; 6],
            pad_last: 0x80,
        };
        let bytes: String = serialize_work(&second_block).to_hex();
        assert_eq!(bytes, "0000000000000001000000000000000200000000000000030000000000000004ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080");
    }
}
