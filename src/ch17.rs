/*
Cryptoptals challenge 17 solution
https://www.cryptopals.com/sets/3/challenges/17
*/

use cryptoctf::symmetric::aes::{OperationMode, AES};
use cryptoctf::padding::pkcs7::pkcs7_pad;
use cryptoctf::operation_modes::oracle::oracle_cbc_padding;
use cryptoctf::encodings::base64::base64_decode;
use rand;
use rand::Rng;

#[allow(dead_code)]
fn get_encrypted() -> Vec<u8> {

    let secrets = vec!["MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
                        "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
                        "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
                        "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
                        "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
                        "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
                        "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
                        "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
                        "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
                        "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93",
                        ];

    let rnd: usize = rand::thread_rng().gen_range(0,secrets.len());

    let key = "\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47".as_bytes().to_owned();
    let iv = [16u8; 16];
    let mut aes = AES::new(&key, OperationMode::CBC{ iv : iv});
    println!("Secret is {}", String::from_utf8(base64_decode(secrets[rnd])).unwrap());
    aes.encrypt(&pkcs7_pad(&base64_decode(secrets[rnd]), 16))

}

#[allow(dead_code)]
fn oracle(data: &[u8]) -> bool {
    let key = "\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47".as_bytes().to_owned();
    let iv = [16u8; 16];
    let mut aes = AES::new(&key, OperationMode::CBC{ iv : iv});
    oracle_cbc_padding(&aes.decrypt(&data))
}

#[cfg(test)]
mod test {

    use super::*;
    use cryptoctf::symmetric::aes_utils::to_blocks;
    use cryptoctf::generic::xor::fixed_xor;


    fn crack_block(b1: &[u8], b2: &[u8]) -> Vec<u8> {
        let mut decrypted = vec![0; 16];
        'outer: for j in 1..17 {
            let pad = vec![vec![0u8; 16 - j], vec![j as u8; j]].concat();
            let mut try = decrypted.clone();
            'inner: for ch in 0..255 {
                try[16 - j] = ch;
                let payload = vec![fixed_xor(&fixed_xor(&pad,&b1), &try), b2.to_vec()].concat();
                if oracle(&payload) {
                    decrypted[16 - j] = ch;
                    continue 'outer;
                }
            }
        }

        return decrypted;
    }


    #[test]
    fn set03_ch17() {
        assert_eq!(oracle_cbc_padding(&vec![16u8; 32]), true);
        assert_eq!(oracle_cbc_padding(&vec![17u8; 16]), false);
     
        let iv = [16u8; 16];
        let encrypted = to_blocks(&get_encrypted());
<<<<<<< HEAD


        for i in 0..encrypted.len() {
            if i == 0 {
                println!("Decrypted block : {:?}", String::from_utf8_lossy(&crack_block(&iv, &encrypted[i])));
            } else {
                println!("Decrypted block : {:?}", String::from_utf8_lossy(&crack_block(&encrypted[i-1], &encrypted[i])));
            }
        }

=======
        let mut total_decrypted = String::new();

        let c = encrypted[0];
        let c_1 = encrypted[1];
        
        /*
        IV, C0, C1
        C0 = E(P0 ^ IV)
        C1 = E(P1 ^ C0)

        P0 = D(C0) ^ IV
        P1 = D(C1) ^ C0

        C = (C0 || C1)
        
        Pour être valide, si len(P) == 31, P_paddé[1] = 0x01
        => donc on BF le dernier byte de 0 a 255 de C0
        On envoie (C0' || C1) au serveur, 
        Si le padding est ok, C0'[-1] ^ 0x01 = D(C1)[-1]
        Sinon on continue

        Ensuite pour le deuxième byte, il faut que P2[-1:-2] == 0x0202
        Donc on forge C0
        */
        
>>>>>>> 5fb2613428813fba5b226a14f57d6c5cdec97c2f
    }
}
