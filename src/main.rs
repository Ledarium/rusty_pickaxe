use log::{debug, info};

use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use pretty_hex::*;

use rustc_hex::{FromHex, ToHex};
use tiny_keccak::{Hasher, Keccak};

use web3::contract::Contract;
use web3::types::{Address, Bytes, H160, H256, U256};

use secp256k1::SecretKey;
use web3::ethabi::{encode, token::Token, FixedBytes};

use serde::{Deserialize, Serialize};

use bincode::Options;
use std::convert::TryInto;

fn vtoa<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[derive(Debug, Serialize, Deserialize)]
struct Work {
    _pad1: [u32; 7],
    chain_id: u32,
    entropy: [u8; 32],
    gem_address: [u8; 20],
    sender_address: [u8; 20],
    _pad2: [u32; 7],
    gem_id: u32,
    _pad3: [u32; 7],
    eth_nonce: u32,
    salt: [u32; 8],
}

#[derive(Debug, Deserialize)]
struct Network {
    chain_id: String, //not used, getting from contract
    rpc: String,
    explorer: String, //not used, not there yet
    gem_address: Address,
}

#[derive(Debug, Deserialize)]
struct Claim {
    private_key: String,
    maximum_gas_price: u32, //not used, not there yet
}

#[derive(Debug, Deserialize)]
struct Config {
    r#loop: bool,
    network: Network,
    gem_type: u32,
    address: Address,
    claim: Claim,
}

fn hash(bytes: Vec<u8>) -> [u8; 32] {
    let mut h = Keccak::v256();
    h.update(&bytes);
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    return res;
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();
    let config_path = std::env::args().nth(1).expect("no config given");
    let mut file = File::open(config_path).unwrap();
    let mut filedata = String::new();
    file.read_to_string(&mut filedata).unwrap();

    let config: Config = serde_json::from_str(&filedata).unwrap();
    debug!("{:?}", config);

    /*
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(2800));
    })
    */

    let transport = web3::transports::Http::new(&config.network.rpc)?;
    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?;
    let prvk = SecretKey::from_str(&config.claim.private_key).unwrap(); // TODO: deserializer

    let contract = Contract::from_json(
        web3.eth(),
        config.network.gem_address,
        include_bytes!("../build/rarity_contract.abi"),
    )
    .unwrap();

    let tx = contract.query("gems", (config.gem_type,), config.address, web3::contract::Options::default(), None);
    /*
    // implement From
    // (https://stackoverflow.com/questions/53194323/is-there-any-way-of-converting-a-struct-to-a-tuple)
    #[derive(Debug)]
    struct GemInfo {
        name: String,
        color: String,
        entropy: Bytes, //2
        difficulty: U256, //3
        gems_per_mine: U256,
        multiplier: U256,
        crafter: Address,
        manager: Address,
        pending_manager: Address,
    }
    */
    let gem_info: (String, String, FixedBytes, U256, U256, U256, Address, Address, Address) = tx.await.unwrap();
    debug!("Got gem_info {:?}", gem_info);

    let eth_nonce = web3.eth().transaction_count(config.address, None).await.unwrap();
    //wow ethabi sucks. just spent 5+hours on figuring this stuff out
    let work = Work {
        _pad1: [0u32; 7],
        chain_id: chain_id.as_u32(),
        entropy: vtoa(gem_info.2),
        _pad2: [0u32; 7],
        gem_id: config.gem_type,
        gem_address: config.network.gem_address.to_fixed_bytes(),
        sender_address: config.address.to_fixed_bytes(),
        _pad3: [0u32; 7],
        eth_nonce: eth_nonce.as_u32(),
        salt: [0u32; 8],
    };
    let bytes = bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&work).unwrap();
    info!("Here is some work for you: {:02X?}", bytes.hex_dump());


    //for i in 0..i64::MAX { }

    Ok(())
}
