/*
Cryptoptals stage 1 challenge 1 solution
https://www.cryptopals.com/sets/1/challenges/1

[antoxyde@anarchy-fixe:Projets/Cryptopals][130]$ rustc ch1.rs
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch1
SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t

*/

pub mod set01;

use set01::hex::hex_to_bytes;
use set01::base64::base64_encode;

fn main() {
    let hexstring = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let s = base64_encode(hex_to_bytes(hexstring));
    println!("{}", s);
}
