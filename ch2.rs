/*
Cryptoptals stage 1 challenge 2 solution
https://www.cryptopals.com/sets/1/challenges/2

Output :

[antoxyde@anarchy-fixe:Projets/Cryptopals]$ rustc ch2.rs
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch2
746865206b696420646f6e277420706c6179

*/
pub mod set01;

use set01::hex::{bytes_to_hex, hex_to_bytes};
use set01::xor::fixed_xor;

fn main() {
    let s1 = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let s2 = hex_to_bytes("686974207468652062756c6c277320657965");
    let s3 = bytes_to_hex(fixed_xor(s1, s2));
    println!("{}", s3);
}
