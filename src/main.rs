use log::{debug, info};

use std::sync::mpsc;
use std::{thread,time};

use std::time::Instant;
use std::fs::File;
use std::io::{Read, Error};
use std::str::FromStr;
use rustc_hex::{FromHex, ToHex};

use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, H160, H256, U256};

use secp256k1::SecretKey;
use bigint::uint::U256 as u256;

mod cuda;

mod cpu;
mod utils;

async fn get_mining_work(
        config: &utils::Config,
        contract: Contract<web3::transports::Http>,
        chain_id: u32,
        end_nonce: u64,
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
    info!("Returning job, target {:?}", target);

    let work = utils::Work {
        first_block: first_block,
        second_block: second_block,
        target: target_bytes,
        start_nonce: 0,
        end_nonce: end_nonce,
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

    let mut config: utils::Config = serde_json::from_str(&filedata).unwrap();
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
    */
    let cuda_enabled = config.cuda;
    if cuda_enabled {
        config.threads = 1;
    }
    if config.threads > 128 {
        println!("wow thats a lot of threads. limiting to 128");
        config.threads = 128;
    }

    loop {
        //let runtime = Runtime::new().unwrap();
        //
        
        let mut channel_work_handles = vec![];
        for tid in 0usize..config.threads {
            let (work_tx, work_rx) = mpsc::channel();
            let (result_tx, result_rx) = mpsc::channel();
            let (hashrate_tx, hashrate_rx) = mpsc::channel();
            &channel_work_handles.push((work_tx, result_rx, hashrate_rx));
            thread::spawn(move || {
                let mut real_salt = u128::MAX;
                while real_salt == u128::MAX {
                    let start_time = Instant::now();
                    let mut work = match work_rx.recv() {
                        Ok(work) => work,
                        Err(e) => break,
                    };
                    let mut result = u64::MAX;
                    if cuda_enabled {
                        if cfg!(feature = "cuda") { result = cuda::mine_cuda(&work); }
                        else { println!("Built without cuda but specified in config"); return }
                    } else {
                        result = cpu::ez_cpu_mine(&work);
                    }
                    if result == u64::MAX {
                        let elapsed = start_time.elapsed();
                        hashrate_tx.send((work.end_nonce - work.start_nonce) as f64/elapsed.as_secs_f64());
                        result_tx.send(u128::MAX);
                        continue;
                    }
                    work.second_block.salt[3] = result;
                    real_salt = work.second_block.get_real_salt();
                    let string_hash: String = cpu::simple_hash(&work).to_hex();
                    //let string_target: String = work.target.to_hex();
                    //debug!("Target: {}", string_target);
                    debug!("Hash(r): {}", string_hash);
                    result_tx.send(real_salt);
                    break;
                }
                info!("[{}] Found salt, waiting for other threads to stop", tid);
            });
        }
        info!("initialized {} threads", channel_work_handles.len());

        for tid_handles in &channel_work_handles {
            for _ in 0..2 {
                let work = get_mining_work(&config.clone(), contract.clone(), chain_id.as_u32(), 10_000_000u64).await.unwrap();
                info!("Sending two initial works");
                tid_handles.0.send(work);
            }
        }
        //println!("Diff is {:?}, nonce {}", gem_info.3, second_block.contract_nonce[3]);

        let mut real_salt = u128::MAX;
        while real_salt == u128::MAX {
            for (tid, tid_handles) in (&channel_work_handles).iter().enumerate() {
                let result = tid_handles.1.try_recv();
                if !result.is_err() {
                    real_salt = result.unwrap();
                    if real_salt == u128::MAX {
                        let thread_hashrate = tid_handles.2.recv().unwrap();
                        println!("[{}] thread hashrate = {:.3}MH/s",
                                 tid,
                                 thread_hashrate/1_000_000f64);
                        let work = get_mining_work(
                            &config.clone(),
                            contract.clone(),
                            chain_id.as_u32(),
                            thread_hashrate as u64
                        ).await.unwrap();
                        tid_handles.0.send(work);
                        info!("No salt found, sending work");
                    } else {
                        println!("Real salt {}", real_salt);
                        break;
                    }
                }
            }
            debug!("All threads working hard, going to sleep now");
            thread::sleep(time::Duration::from_millis(100));
        }
                
        let prvk = SecretKey::from_str(&config.claim.private_key).unwrap(); // TODO: deserializer
        let tx = contract.signed_call_with_confirmations(
            "mine", (config.gem_type, real_salt), web3::contract::Options::default(), 1, &prvk);
        for tid_handles in &channel_work_handles {
            drop(&tid_handles.0);
        }
        let tx_result = tx.await.unwrap();
        println!("Sent TX: {}tx/{:?}", config.network.explorer, tx_result.transaction_hash);
        /*
        let tx = contract
            .call("mine", (config.gem_type, result), config.address, Options::default())
            .await
            .unwrap();
        // Sign the tx (can be done offline)
        //let signed = web3.accounts().sign_transaction(tx, &prvk).await?;
        //
        */
        if !config.r#loop { break; }
    };
     
    Ok(())
}
