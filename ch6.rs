/*
Cryptoptals stage 1 challenge 6 solution
https://www.cryptopals.com/sets/1/challenges/6
*/
pub mod utils;

use std::fs::File;
use std::io::prelude::*;

use utils::base64::base64_decode;
use utils::others::{hamming_distance, m_split};
fn main() {
    assert_eq!(
        hamming_distance("this is a test".to_string(), "wokka wokka!!!".to_string()),
        37
    );

    let file = File::open("6.txt");
    let mut encoded_content = String::new();
    file.unwrap().read_to_string(&mut encoded_content);

    let content: String =
        String::from_utf8(base64_decode(encoded_content.replace("\n", ""))).unwrap();

    let mut keysize_guess: Vec<(i32, i32)> = Vec::new();

    for guess in 2..40 as usize {
        let average_hamming_distance = (hamming_distance(
            content.get(0..guess).unwrap().to_string(),
            content.get(guess..2 * guess).unwrap().to_string(),
        )
            + hamming_distance(
                content.get(2 * guess..3 * guess).unwrap().to_string(),
                content.get(3 * guess..4 * guess).unwrap().to_string(),
            )) / (2 * guess as i32);
        keysize_guess.push((guess as i32, average_hamming_distance));
    }

    keysize_guess.sort_by(|a, b| a.1.cmp(&b.1)); //Sort by lower hamming distance
    let possible_keysize: Vec<i32> = keysize_guess.iter().map(|a| a.0).take(5).collect(); //Take the five lowest ones

    println!("[*]Five most probables keysizes are {:?}", possible_keysize);

    let mut zipped: Vec<String> = Vec::new();
    for ks in possible_keysize {
        let m = m_split(&content, ks as usize);
        for x in 0..ks {
            for y in 0..m.len() {
                if m[y as usize].len() == ks as usize {
                    zipped[x as usize].push(m[y as usize].chars().nth(x as usize).unwrap());
                }
            }
        }
    }

    println!("{:?}", zipped);

    //println!("{}", content);
}
