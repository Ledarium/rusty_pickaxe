use log::{debug, info};

use std::sync::mpsc;
use std::fs::File;
use std::io::{Read, Error};
use std::str::FromStr;
use rustc_hex::{FromHex, ToHex};

use tokio::runtime::Runtime;
use futures::executor::block_on;

use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, H160, H256, U256};

use secp256k1::SecretKey;
use bigint::uint::U256 as u256;


mod cpu;
mod utils;

async fn get_mining_work(
        config: &utils::Config, contract: Contract<web3::transports::Http>, chain_id: u32
    ) -> Result<utils::Work, Error> {
    let tx = contract.query("gems", (config.gem_type,), config.address, Options::default(), None); 
    /*
    // implement From or drop this
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
    let entropy = utils::vtoa(gem_info.2);

    let tx = contract.query("nonce", (config.address,), config.address, Options::default(), None);
    let contract_nonce_tx: (U256, ) = tx.await.unwrap();

    //wow ethabi sucks. just spent 5+hours on figuring this stuff out
    let first_block = utils::WorkFirstBlock {
        _pad1: [0; 7],
        chain_id: chain_id,
        entropy: entropy,
        _pad2: [0; 7],
        gem_id: config.gem_type,
        gem_address: config.network.gem_address.to_fixed_bytes(),
        sender_address: config.address.to_fixed_bytes(),
    };

    let mut second_block = utils::WorkSecondBlock {
        contract_nonce: [0, 0, 0, contract_nonce_tx.0.as_u64()],
        salt: [0; 4],
        pad_first: 0x01, // see keccak specifications for explaination
        zero_pad0: [0; 8],
        zero_pad1: [0; 6],
        pad_last: 0x80,
    };
    second_block.randomize_salt();

    let target = u256::max_value() / u256::from(gem_info.3.as_u64());
    let mut target_bytes = [0u8; 32];
    target.to_big_endian(&mut target_bytes);
    println!("Diff is {:?}, nonce {}", gem_info.3, second_block.contract_nonce[3]);
    info!("Returning job, target {:?}", target);

    let work = utils::Work {
        first_block: first_block,
        second_block: second_block,
        target: target_bytes,
        start_nonce: 0,
        end_nonce: u64::MAX,
    };
    Ok(work)
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();
    let config_path = std::env::args().nth(1).expect("no config given");
    let mut file = File::open(config_path).unwrap();
    let mut filedata = String::new();
    file.read_to_string(&mut filedata).unwrap();

    let config: utils::Config = serde_json::from_str(&filedata).unwrap();
    debug!("{:?}", config);

    let transport = web3::transports::Http::new(&config.network.rpc)?;
    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?;

    let contract = Contract::from_json(
        web3.eth(),
        config.network.gem_address,
        include_bytes!("../build/rarity_contract.abi"),
    )
    .unwrap();

    /*
    tokio::task::spawn_blocking(|| {
        std::thread::sleep(Duration::from_millis(2800));
    })
    let (tx, rx) = mpsc::channel();
    */

    loop {
        let runtime = Runtime::new().unwrap();
        let mut work = get_mining_work(&config, contract.clone(), chain_id.as_u32()).await.unwrap();
        let result = cpu::ez_cpu_mine(&work);
        work.second_block.salt[3] = result;
        let real_salt = work.second_block.get_real_salt();

        println!("Real salt {}", real_salt);
        let string_hash: String = cpu::simple_hash(&work).to_hex();
        debug!("Hash(r): {}", string_hash);

        /*
        let tx = contract
            .call("mine", (config.gem_type, result), config.address, Options::default())
            .await
            .unwrap();
        // Sign the tx (can be done offline)
        //let signed = web3.accounts().sign_transaction(tx, &prvk).await?;
        //
        */
        /*
        let prvk = SecretKey::from_str(&config.claim.private_key).unwrap(); // TODO: deserializer
        let tx = contract.signed_call_with_confirmations(
            "mine", (config.gem_type, real_salt), web3::contract::Options::default(), 1, &prvk)
            .await
            .unwrap();
        info!("Sent TX: {:?}",tx);
        */
        if !config.r#loop { break; }
    };
     
    Ok(())
}
