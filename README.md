# Rusty Pickaxe

Multithreaded CPU miner for [Provably Rare Gems](https://gems.alphafinance.io/#/rarity), written in Rust.
There is also closed-source GPU version, waiting to be released.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
`loop` option works since v0.0.2. `threads` options works and is required since v0.0.4.
You will probably want to set `threads` equal or less than your CPUs thread count.

## Prerequisites

If following steps do not work for you, you can always ask for support in [Discord](https://discord.gg/xDk6enpGnM).

### Ubuntu

Install rust: https://www.rust-lang.org/tools/install

Install extra bits: `sudo apt install build-essential libssl-dev pkg-config`

### Windows

Install VS build tools: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 ,
you want "Build Tools for Visual Studio 2019". When the installer asks what parts of 
"Build Tools for Visual Studio 2019" you want, go to the "individual components" tab
and pick "x64/x86 build tools (latest)".

Install rust: https://www.rust-lang.org/tools/install

## Run

`cargo run --release config.json`

## Hashrate test

Data is stated in MH/s.
You can share your results to appear here.

CPU | Single core | Threads | Multi core
--- | --- | --- | ---
Ryzen 3700X | 2.67 | 16 | ~18.70
Ryzen 2600 | 2.20 | 12 | ~13.44

## Improvement plan (ordered by priority)

1. Proper error handling
2. Better UI
 
## Acknowledgments

- Made it public thanks to generous `wenl#6575`.
- `Kakapo#5409` made nice json format for config file and wrote install instructions.
- Also thanks to `Spiry#6864`, `spiz0r#7566` for donations.
- Anonymous donations were also a big help.

Kudos to all of you! Donations are accepted here: [Ethereum](https://etherscan.io/address/0x8dd47bf52589cf12ff4703951c619821cf794b77), [Fantom](https://ftmscan.com/address/0x8dd47bf52589cf12ff4703951c619821cf794b77) .

## Contacts

- Discord `Booba#1974`
- Telegram @amazing_booba

