/*
Cryptoptals challenge 11 solution
https://www.cryptopals.com/sets/2/challenges/11
*/
use cryptoctf::symmetric::aes::{AES, OperationMode};
use cryptoctf::symmetric::aes_utils::gen_rnd_128_bits;

use rand;
use rand::Rng;

use cryptoctf::padding::pkcs7::pkcs7_pad;

#[allow(dead_code)]
fn rnd_encrypt(input: &[u8]) -> (Vec<u8>,OperationMode)  {

    let mut rng = rand::thread_rng();

    let key = gen_rnd_128_bits();

    let mut data: Vec<u8> = input.clone().to_owned();

    data.reverse();

    for _ in 0..rng.gen_range(5, 10) { //Append 5 to 10 bytes at the start of the data
        data.push(rng.gen::<u8>());
    }

    data.reverse();

    for _ in 0..rng.gen_range(5, 10) { //Append 5 to 10 bytes at the end of the data
        data.push(rng.gen::<u8>());
    }

    let mode = match rng.gen() { //Choose a random OperationMode
        true => OperationMode::ECB,
        false => OperationMode::CBC{iv : gen_rnd_128_bits()},
    };

    let mut aes = AES::new(&key, mode);

    let padded_data = pkcs7_pad(&data, 16);

    return (aes.encrypt(&padded_data), mode);
}



#[cfg(test)]
mod test {

    use super::rnd_encrypt;

    use cryptoctf::operation_modes::oracle::oracle_ecb_cbc;
    use cryptoctf::symmetric::aes::OperationMode;

    #[test]
    fn set02_ch11() {

        let data = vec![0u8; 44];

        for _ in 0..100 {

            let (encrypted, mode) = rnd_encrypt(&data);
            let guessed_mode = oracle_ecb_cbc(&encrypted);

            assert!((guessed_mode == "ECB" && mode == OperationMode::ECB) || (guessed_mode == "CBC" && mode != OperationMode::ECB));
        }

    }
}
