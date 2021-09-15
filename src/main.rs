extern crate tiny_keccak;
extern crate rustc_hex;

use rustc_hex::{FromHex, ToHex};
use tiny_keccak::{Hasher, Keccak};

fn main() {
    
    /*
    let mut h = Keccak::v256();
    let mut output = [0u8; 32];
    let expected = b"\
        \x64\x4b\xcc\x7e\x56\x43\x73\x04\x09\x99\xaa\xc8\x9e\x76\x22\xf3\
        \xca\x71\xfb\xa1\xd9\x72\xfd\x94\xa3\x1c\x3b\xfb\xf2\x4e\x39\x38\
    ";

    h.update(b"hello world");
    h.finalize(&mut output);
    assert_eq!(expected, &output);
    */

    let hex = "00000000000000000000000000000000000000000000000000000000000000ea";
    println!("{}", hex);

    let bytes: Vec<u8> = hex.from_hex().unwrap();

    let mut h = Keccak::v256();
    h.update(&bytes);
    let mut res = [0u8; 32];
    h.finalize(&mut res);
    let out: String = res.to_hex();

    println!("{:?}", out);
}
