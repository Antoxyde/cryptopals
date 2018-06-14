/*
Cryptoptals stage 2 challenge 6 solution
https://www.cryptopals.com/sets/2/challenges/6
*/

use libs::base64::base64_decode;
use libs::aes::{OperationMode, AES};

use libs::padding::pkcs7_pad;




#[allow(dead_code)]
fn m_encrypt(input: &[u8]) -> Vec<u8>  {

    let key = b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47";
    let prepend = b"\xa8\x4f\x3a\x6f\xeb\x7b\xce\xf3\xe8\x5a\x84\xe0\x06\xb7\x85\xaf\x05\x3b\x03\x4c\xfd\x9c\xcd\x91\xc5\x79";
    let b64data = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                    YnkK";

    let mut data: Vec<u8> = prepend.clone().to_owned().to_vec();

    //Todo use append/extend
    for c in input {
        data.push(*c);
    }

    for c in base64_decode(b64data) {
        data.push(c);
    }

    let mut aes = AES::new(key, OperationMode::ECB);
    let padded_data = pkcs7_pad(&data, 16);

    return aes.encrypt(&padded_data);
}
