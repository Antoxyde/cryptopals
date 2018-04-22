/*
Cryptoptals stage 1 challenge 3 solution
https://www.cryptopals.com/sets/1/challenges/3

Simply try every possible char to break xor and if their is more than 80% of ascii char and space , it's probably a valid string.

Output :
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ rustc ch3.rs
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch3
Cooking MC's like a pound of bacon

*/

pub mod include;

use include::xor::crack_xor;
use include::hex::hex_to_bytes;

fn main() {
    let s = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let pr = crack_xor(s);
    println!("{}", pr.unwrap());
}
