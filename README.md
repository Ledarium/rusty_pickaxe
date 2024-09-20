# Rusty Pickaxe

Multithreaded CPU and GPU (CUDA) miner for [Provably Rare Gems](https://gems.alphafinance.io/#/rarity), written in Rust.

## Config

Same `config.json` as in [ramen](https://github.com/dmptrluke/ramen) miner. 
`loop` option works since v0.0.2. `threads` options works and is required since v0.0.4.
You will probably want to set `threads` equal or less than your CPUs thread count.
If you want to mine with cuda set `threads` to anything, it does not matter.

## Binaries (untested)

Grab one from [releases page](https://github.com/Ledarium/rusty_pickaxe/releases).
Run it like `rusty_pickaxe.exe config.json`

## Build from source
### Prerequisites

If following steps do not work for you, you can always ask for support in [Discord](https://discord.gg/xDk6enpGnM).

#### Ubuntu

For cuda install refer to https://docs.nvidia.com/cuda/cuda-installation-guide-linux/index.html

Install rust: https://www.rust-lang.org/tools/install

Install extra bits: `sudo apt install build-essential libssl-dev pkg-config`

#### Windows

For cuda install refer to https://docs.nvidia.com/cuda/cuda-installation-guide-microsoft-windows/index.html

Install VS build tools: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 ,
you want "Build Tools for Visual Studio 2019". When the installer asks what parts of 
"Build Tools for Visual Studio 2019" you want, go to the "individual components" tab
and pick "x64/x86 build tools (latest)".

Install rust: https://www.rust-lang.org/tools/install

#### CUDA version

If you are getting errors like this:
```
cargo:warning=nvcc fatal   : Unsupported gpu architecture 'compute_86'
```
This means your GPU is not enabled in config. I did enable most popular ones like 
9XX/10XX, RTX 20XX/30XX. But fully enabling all of them does not make sense and could
negatively affect performance. If you are having this issue, see your `compute_XX` and
`sm_XX` params 
[here](https://arnon.dk/matching-sm-architectures-arch-and-gencode-for-various-nvidia-cards/)
and write them replacing predefined ones in `src/build.rs`.

### Run

CPU: `cargo run --release config.json`

CUDA: `cargo run --features cuda --release config.json`

Only one GPU in system is supported for now. However, you can launch multiple miners
changing `N` for every card in system.

- Linux: `CUDA_VISIBLE_DEVICES=N cargo ...`
- Windows (untested): `set CUDA_VISIBLE_DEVICES=N & cargo ...`

## Hashrate test

Data is stated in MH/s.
You can share your results to appear here.

CPU | Single core | Threads | Multi core
--- | --- | --- | ---
Ryzen 3700X | 2.67 | 16 | ~18.70
Ryzen 2600 | 2.20 | 12 | ~13.44
i5-9600k@5GHz | 2.82 | 6 | ~16.92
i7-9700@3GHz | 2.08 | 8 | ~16.64
i7-10700k@5GHz | 2.50 | 8 | ~20.00

GPU | Hashrate
--- | ---
3090 | 1700
3070 | 1007
3060 Ti | 1007
2080 Ti | 1200
1080 Ti | 787
1060 | 280...324
1050 Ti | 160...183
1050 | 154
1030 | 89
 
## Acknowledgments

- Made it public thanks to generous `wenl#6575`.
- `Kakapo#5409` made nice json format for config file and wrote install instructions.
- Also thanks to `Spiry#6864`, `spiz0r#7566`, `shock#9999` for donations.
- Thanks to Alpha Finance for sponsoring. Wish I could apply for GPU grant though.
- Anonymous donations were also a big help.

## Legal notice

SOFTWARE IS PROVIDED AS IS AND AUTHOR IS NOT RESPONSIBLE FOR ANYTHING.
