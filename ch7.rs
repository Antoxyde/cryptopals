/*
Cryptoptals stage 1 challenge 7 solution
https://www.cryptopals.com/sets/1/challenges/7
*/

pub mod include;

use include::aes::aes_encrypt;

fn main() {
    let key = "YELLOW SUBMARINE".as_bytes().to_owned();
    let inb = "Attack at Dawn!!".as_bytes().to_owned();
    let mut outb: Vec<u8> = Vec::new();
    aes_encrypt(key, &inb, &mut outb);
    println!("Encrypted block : {:?}", outb);
}
