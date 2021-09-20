# Rusty Pickaxe


Single core for now. If i recieve grant (or if i get bored), i will parallelize it.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
Also `loop` option is always active by now, but I think that's okay?

## Prerequisites

If following steps do not work for you, you can always ask for support in [Discord](https://discord.gg/xDk6enpGnM).

### Ubuntu

Install rust: https://www.rust-lang.org/tools/install

Install extra bits: `sudo apt install build-essential libssl-dev pkg-config`

### Windows

Install VS build tools: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 ,
you want "Build Tools for Visual Studio 2019". When the installer asks what parts of 
"Build Tools for Visual Studio 2019" you want, go to the "individual components" tab
and pick "x64/x86 build tools (latest).

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
3. Multithreaded CPU mining
4. Better UI
 
## Acknowledgments

- Made it public thanks to generous `wenl#6575`.
- `Spiry#6864` helped to develop multithreading.
- `Kakapo#5409` made nice json format for config file and wrote install instructions.
- Anonymous donations were also a big help.

Kudos to all of you! Donations are accepted here: [Ethereum](https://etherscan.io/address/0x8dd47bf52589cf12ff4703951c619821cf794b77), [Fantom](https://ftmscan.com/address/0x8dd47bf52589cf12ff4703951c619821cf794b77) .

## Contacts

- Discord `Booba#1974`
- Telegram @amazing_booba

