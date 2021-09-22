# Rusty Pickaxe


Single core for now. If i recieve grant (or if i get bored), i will parallelize it.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
`loop` option works since v0.0.2.

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

Mining is no longer sequential, so you can run it multiple times, until proper multithreading
is implemented.

## Hashrate test

You can share your results to appear here.

Ryzen 3700X = 2.67 MH/s

Ryzen 2600  = 2.20 MH/s

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

