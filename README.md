# Rusty Pickaxe

Made it public thanks to generous Discord user `wenl#6575`. Kudos!

Single core for now. If i recieve grant (or if i get bored), i will parallelize it.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
Also `loop` option is always active by now, but I think that's okay?

## Build

### Ubuntu

Install rust: https://www.rust-lang.org/tools/install

Install extra bits: `sudo apt install build-essential libssl-dev pkg-config`

### Windows

Install rust: https://www.rust-lang.org/tools/install

## Run

`cargo run --release config.json`

Please note that mining is sequential, so there is no reason to run it multiple times -
results will be the same and you will get rejected transactions. Need to implement proper
scheduling to get it right and to work with GPU.

## Hashrate test

You can share your results to appear here.

Ryzen 3700X = 2.67 MH/s

Ryzen 2600  = 2.20 MH/s

## Improvement plan (ordered by priority)

1. First priority is GPU miner MWE (minimal working example) based on this code
2. Proper scheduling to allow multiple threads run simultaneously
3. Multithreaded CPU mining [donation target is 800$]
4. Better UI
 
## Contacts

- Donations are accepted here: [0x8dd47bf52589cf12ff4703951c619821cf794b77](https://etherscan.io/address/0x8dd47bf52589cf12ff4703951c619821cf794b77)
- Discord Booba#1974
- Telegram @amazing_booba

