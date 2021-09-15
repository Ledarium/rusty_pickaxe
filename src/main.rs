extern crate tiny_keccak;
extern crate rustc_hex;
extern crate web3;

use std::str::FromStr;
use rustc_hex::{FromHex, ToHex};
use tiny_keccak::{Hasher, Keccak};

use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, H256, U256};

use secp256k1::SecretKey;
use web3::ethabi::{encode, token::Token};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let hex = "00000000000000000000000000000000000000000000000000000000000000ea";
    println!("{}", hex);
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

    let transport = web3::transports::Http::new("https://rpc.ftm.tools")?;
    let web3 = web3::Web3::new(transport);
    let chain_id = web3.eth().chain_id().await?;
    // Insert the 20-byte "to" address in hex format (prefix with 0x)
    let my_addr = Address::from_str();
    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str().unwrap();

    let contract_address: Address = "0x2a9Cbf31717854Ad005EA4FcCB573c20eA43e036".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../build/rarity_contract.abi"),
    )
    .unwrap();

    let work = encode(&[Token::Uint(chain_id)]); 

    Ok(())
}
