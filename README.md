# Rusty Pickaxe

Single core for now. If i recieve grant (or if i get bored), i will parallelize it.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
`loop` option works since v0.0.2.
Need to add `"cuda": true` (see example in repo), otherwise it fails.

## Prerequisites

If following steps do not work for you, you can always ask for support in [Discord](https://discord.gg/xDk6enpGnM).

### Ubuntu

For cuda install refer to https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html

Install rust: https://www.rust-lang.org/tools/install

Install extra bits: `sudo apt install build-essential libssl-dev pkg-config`

### Windows

For cuda install refer to https://docs.nvidia.com/cuda/cuda-installation-guide-microsoft-windows/index.html

Install VS build tools: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 ,
you want "Build Tools for Visual Studio 2019". When the installer asks what parts of 
"Build Tools for Visual Studio 2019" you want, go to the "individual components" tab
and pick "x64/x86 build tools (latest).

Install rust: https://www.rust-lang.org/tools/install

## Run

`cargo run --release config.json`

Please note that CPU mining is sequential, so there is no reason to run it multiple times -
results will be the same and you will get rejected transactions.

- GPU is randomized so it should work properly ONLY if you use multiple wallets on multiple devices.
- Only one GPU in system is supported.
- Mining for one wallet on multiple devices is not supported, YOU WILL get rejected TXs.

Need to implement proper scheduling and difficulty polling to prevent rejected TXs,
both with CPU and GPU. This is next target.

## Hashrate test

You can share your results to appear here.

Ryzen 3700X = 2.67 MH/s

Ryzen 2600  = 2.20 MH/s

## Improvement plan (ordered by priority)

1. Proper scheduling to allow multiple threads run simultaneously and difficulty polling to
prevent rejects
2. Multithreaded CPU mining
3. Better UI
 
## Acknowledgments

- Made it public thanks to generous `wenl#6575`.
- `Spiry#6864` helped to develop multithreading.
- `Kakapo#5409` made nice json format for config file and wrote install instructions.
- Anonymous donations were also a big help.

Kudos to all of you! Donations are accepted here: [Ethereum](https://etherscan.io/address/0x8dd47bf52589cf12ff4703951c619821cf794b77), [Fantom](https://ftmscan.com/address/0x8dd47bf52589cf12ff4703951c619821cf794b77) .

## Contacts

- Discord `Booba#1974`
- Telegram @amazing_booba

## Legal notice

SOFTWARE IS PROVIDED AS IS AND AUTHOR IS NOT RESPONSIBLE FOR ANYTHING.
