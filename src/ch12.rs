/*
Cryptoptals stage 2 challenge 4 solution
https://www.cryptopals.com/sets/2/challenges/4
*/
use libs::base64::base64_decode;
use libs::aes::{OperationMode, AES};

use libs::padding::pkcs7_pad;

#[allow(dead_code)]
fn m_encrypt(input: &[u8]) -> Vec<u8>  {

    let key = b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47"; //head -n 1 /dev/urandom |xxd => first line

    let mut data: Vec<u8> = input.clone().to_owned();

    let base64_data = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                    aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                    dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                    YnkK";

    let append_data = base64_decode(base64_data);

    for c in append_data {
        data.push(c);
    }

    let mut aes = AES::new(key, OperationMode::ECB);
    let padded_data = pkcs7_pad(&data, 16);

    return aes.encrypt(&padded_data);
}


#[cfg(test)]
mod test {

    use super::m_encrypt;
    use libs::aes_utils::{is_ecb,to_blocks};

    #[test]
    fn ch12() {

        let mut data: Vec<u8> = Vec::new();

        let encrypted_data_size = m_encrypt(&data).len();
        let mut actual_data_size = encrypted_data_size;

        while encrypted_data_size == actual_data_size {
            data.push(0);
            actual_data_size = m_encrypt(&data).len();
        }

        let block_size  = actual_data_size - encrypted_data_size;

        assert_eq!(block_size, 16);

        let data = vec![0; 50];

        let encrypted = m_encrypt(&data);
        let blocks = to_blocks(&encrypted);

        assert!(is_ecb(&blocks));

        let mut data = vec![0; 16];

        for x in (0..16).rev() {

            let mut codebook: Vec<[u8; 16]> = Vec::new();

            //Generate the codebook
            for c in 0..126 {
                let mut plain = data.clone();
                plain[x] = c;

                println!("Encrypting {:?}", plain);
                let encrypted = m_encrypt(&plain);
                let mut block =  [0u8; 16];
                block.copy_from_slice(&encrypted[0..16]);

                codebook.push(block);
            }

            let encrypted = m_encrypt(&data);
            let mut block =  [0u8; 16];
            block.copy_from_slice(&encrypted[0..16]);

            for y in 0..126 as u8 {
                if codebook[y as usize] == block {
                    println!("Letter at place {} is {}", x, y as char);
                    data[x] = y;
                }
            }

        }

        let result = String::from_utf8(data).unwrap();
        println!("First block was {}", result);



    }
}
