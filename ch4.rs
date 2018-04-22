/*
Cryptoptals stage 1 challenge 4 solution
https://www.cryptopals.com/sets/1/challenges/4

Simply try every possible char to break xor on every hexstring of the file and if their is more than 90% of ascii char and space , it's probably the valid string.

Output :
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ rustc ch4.rs
warning: unused `std::result::Result` which must be used
  --> ch4.rs:20:5
   |
20 |     file.unwrap().read_to_string(&mut content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default

[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch4
Now that the party is jumping

*/

pub mod include;

use include::hex::hex_to_bytes;
use include::xor::crack_xor;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("4.txt");
    let mut content = String::new();
    file.unwrap().read_to_string(&mut content);
    for try in content.split("\n") {
        match crack_xor(hex_to_bytes(try)) {
            Some(s) => {
                println!("{}", s); //note : dunno why there is a line feed here
                return;
            }
            None => continue,
        }
    }
}
