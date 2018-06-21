/*
Cryptoptals challenge 14 solution
https://www.cryptopals.com/sets/2/challenges/14
*/

use libs::aes::{OperationMode, AES};

use libs::padding::pkcs7_pad;


#[allow(dead_code)]
fn m_encrypt(prefix: &[u8], input: &[u8], suffix: &[u8]) -> Vec<u8>  {

    let key = b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47";

    let data = vec![prefix.to_vec(), input.to_vec(), suffix.to_vec()].concat();
    let mut aes = AES::new(key, OperationMode::ECB);
    let padded_data = pkcs7_pad(&data, 16);
    return aes.encrypt(&padded_data);
}

#[cfg(test)]
mod test {

    use super::*;
    use libs::base64::base64_decode;
    use rand;
    use rand::{Rng, RngCore};

    #[test]
    fn ch14() {

        let mut rng = rand::thread_rng();

        let suffix = base64_decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                        aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                        dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                        YnkK");
        
        let mut prefix = vec![0u8; rng.gen_range(0,16)];
        rng.fill_bytes(&mut prefix);
        let base_ct = m_encrypt(&prefix, &vec![], &suffix);
        let mut padlen = 16;

        for i in 0..16 {
            let try = m_encrypt(&prefix, &vec![0u8; i] , &suffix);
            if try.len() != base_ct.len() {
                padlen = i;
                break;
            }
        }

        let prefix_len = base_ct.len()- suffix.len()  - padlen;
        assert_eq!(prefix_len, prefix.len());
        let block_size = 16;
        let offset = block_size - prefix_len;

        let mut recovered = String::new();
        let size_guess_unknown_string = 144;
        let mut data = vec![0; offset + size_guess_unknown_string - 1 ];
        let mut codebook_data = data.clone();

        for _ in 0..size_guess_unknown_string {

            let mut codebook: Vec<[u8; 16]> = Vec::new();

            for c in 0..126 {

                let mut plain = codebook_data.clone();
                plain.push(c);
                let encrypted = m_encrypt(&prefix, &plain, &suffix);
                let mut block =  [0u8; 16];
                block.copy_from_slice(&encrypted[size_guess_unknown_string..size_guess_unknown_string+block_size]);
                codebook.push(block);

            }

            let encrypted = m_encrypt(&prefix, &data, &suffix);
            let mut block =  [0u8; 16];
            block.copy_from_slice(&encrypted[size_guess_unknown_string..size_guess_unknown_string+block_size]);
            let mut letter = 0u8;

            for y in 0..126 as u8 {
                if codebook[y as usize] == block {
                    letter = y;
                    break;
                }
            }

            if letter == 1 {
                break;
            }

            data.reverse();
            data.pop();
            data.reverse();

            codebook_data.reverse();
            codebook_data.pop();
            codebook_data.reverse();
            codebook_data.push(letter);

            recovered.push(letter as char);
        }

        assert_eq!(recovered.into_bytes(), suffix);

    }
}
