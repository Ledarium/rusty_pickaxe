use tiny_keccak::{Hasher, Keccak};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use web3::types::Address;
use bincode::Options;

pub fn prepare_data(pre_work: &PreWork) -> Keccak {
    let mut h = Keccak::v256();
    let bytes = serialize_work(pre_work);
    h.update(&bytes);
    h.update(&[0u8; 16]); //salt high bits
    return h;
}

pub fn serialize_work(pre_work: &PreWork) -> Vec<u8> {
    return bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreWork {
    pub _pad1: [u32; 7],
    pub chain_id: u32,
    pub entropy: [u8; 32],
    pub gem_address: [u8; 20],
    pub sender_address: [u8; 20],
    pub _pad2: [u32; 7],
    pub gem_id: u32,
    pub _pad3: [u32; 7],
    pub eth_nonce: u32,
    //next 256 bits are salt. we define them in miner struct
}

#[derive(Debug, Deserialize)]
pub struct Network {
    pub chain_id: String, //not used, getting from contract
    pub rpc: String,
    pub explorer: String, //not used, not there yet
    pub gem_address: Address,
}

#[derive(Debug, Deserialize)]
pub struct Claim {
    pub private_key: String,
    pub maximum_gas_price: u32, //not used, not there yet
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub r#loop: bool,
    pub network: Network,
    pub gem_type: u32,
    pub address: Address,
    pub claim: Claim,
}

pub fn vtoa<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[cfg(test)]
mod tests {
    use bincode::Options;
    use crate::utils::{PreWork, serialize_work};
    use web3::types::Address;
    use std::str::FromStr;
    use rustc_hex::ToHex;

    #[test]
    fn test_bin_data() {
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
        let bytes: String = serialize_work(&ex_work).to_hex();
        assert_eq!(bytes, "00000000000000000000000000000000000000000000000000000000000000016262626262626262626262626262626262626262626262626262626262626262ffcf8fdee72ac11b5c542428b35eef5769c409f090f8bf6a479f320ead074411a4b0e7944ea8c9c100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000014");
    }
}
