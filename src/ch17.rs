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

    #[test]
    fn set03_ch17() {
        assert_eq!(oracle_cbc_padding(&vec![16u8; 32]), true);
        assert_eq!(oracle_cbc_padding(&vec![17u8; 16]), false);

        let encrypted = to_blocks(&get_encrypted());
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
        
    }
}
