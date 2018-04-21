/*

Cryptoptals stage 1 challenge 5 solution
https://www.cryptopals.com/sets/1/challenges/5

Juste a simple key-repeating xor.

*/

pub mod utils;

use utils::hex::bytes_to_hex;
use utils::xor::key_cycling_xor;

fn main() {
    let plain =
        "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
    let key = "ICE".as_bytes();
    let result = bytes_to_hex(key_cycling_xor(&plain, &key));
    assert_eq!(result, "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    println!("{}", result);
}
