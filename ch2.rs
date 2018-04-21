pub mod utils;

use utils::hex::{bytes_to_hex, hex_to_bytes};
use utils::crypto::fixed_xor;

fn main() {
    let s1 = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let s2 = hex_to_bytes("686974207468652062756c6c277320657965");
    let s3 = bytes_to_hex(fixed_xor(s1, s2));
    println!("{}", s3);
}
