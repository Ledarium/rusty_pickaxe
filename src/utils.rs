use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use web3::types::Address;

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
