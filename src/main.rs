use log::{debug, info};

use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use pretty_hex::*;

use std::mem::transmute;
use rustc_hex::{FromHex, ToHex};

use web3::contract::Contract;
use web3::types::{Address, Bytes, H160, H256, U256};

use secp256k1::SecretKey;

use bincode::Options;

mod cpu;
mod utils;

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();
    let config_path = std::env::args().nth(1).expect("no config given");
    let mut file = File::open(config_path).unwrap();
    let mut filedata = String::new();
    file.read_to_string(&mut filedata).unwrap();

    let config: utils::Config = serde_json::from_str(&filedata).unwrap();
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
    let gem_info: (String, String, Vec<u8>, U256, U256, U256, Address, Address, Address) = tx.await.unwrap();
    debug!("Got gem_info {:?}", gem_info);

    let eth_nonce = web3.eth().transaction_count(config.address, None).await.unwrap();

    //wow ethabi sucks. just spent 5+hours on figuring this stuff out
    let pre_work = utils::PreWork {
        _pad1: [0u32; 7],
        chain_id: chain_id.as_u32(),
        entropy: utils::vtoa(gem_info.2),
        _pad2: [0u32; 7],
        gem_id: config.gem_type,
        gem_address: config.network.gem_address.to_fixed_bytes(),
        sender_address: config.address.to_fixed_bytes(),
        _pad3: [0u32; 7],
        eth_nonce: eth_nonce.as_u32(),
    };
    let bytes = bincode::options()
        .with_fixint_encoding()
        .allow_trailing_bytes()
        .with_big_endian()
        .serialize(&pre_work).unwrap();

    /*
    info!("Here is some work for you: {:?}", owork.data.hex_dump());
    let hash: String = optimized_hash(owork).to_hex();
    info!("Here is hash {:?}", hash);
    */

    let mut owork = cpu::OptimizedWork {
        data: bytes,
        salt_high: 0u128,
        salt_low: 0u128,
        target: [0xFFu8; 32],
    };
    
    println!("Diff is {:?}", gem_info.3);
    let target = div_up(u128::MAX, gem_info.3.as_u128()).to_be_bytes();
    for i in 16..32 {
        owork.target[i] = target[i-16]
    }
//    owork.target = 
    let result = cpu::ez_cpu_mine(&mut owork);
    println!("Here is salt {:?}", result);
    Ok(())
}

pub fn div_up(a: u128, b: u128) -> u128 {
    return a / b + (a % b != 0) as u128;
}

