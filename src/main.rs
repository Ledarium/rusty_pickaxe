extern crate tiny_keccak;
extern crate rustc_hex;
extern crate web3;

use log::{debug, error, log_enabled, info, Level};

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use rustc_hex::{FromHex, ToHex};
use tiny_keccak::{Hasher, Keccak};

use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, H256, U256};

use secp256k1::SecretKey;
use web3::ethabi::{encode, token::Token};

use serde::Deserialize;
use serde_json::{Result, Number};

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

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();
    let mut file = File::open("config.json").unwrap();
    let mut filedata = String::new();
    file.read_to_string(&mut filedata).unwrap();

    let config: Config = serde_json::from_str(&filedata).unwrap();
    debug!("{:?}", config);

    let hex = "00000000000000000000000000000000000000000000000000000000000000ea";
    info!("{}", hex);
    let bytes: Vec<u8> = hex.from_hex().unwrap();
    let mut h = Keccak::v256();
    h.update(&bytes);
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    let out: String = res.to_hex();
    println!("{}", out);

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

    /*
    (
        name,
        color,
        entropy,
        difficulty,
        gemsPerMine,
        multiplier,
        crafter,
        manager,
        pendingManager,
    ) = network.functions.gems(args.gem).call()
    (chain_id, entropy, gemAddr, senderAddr, kind, nonce, salt)
    ["uint256", "bytes32", "address", "address", "uint", "uint", "uint"]
    */
    let tx = contract.call("gems", config.gem_type, config.address, Options::default()).await?;
    let work = encode(&[Token::Uint(chain_id)], ); 

    Ok(())
}
