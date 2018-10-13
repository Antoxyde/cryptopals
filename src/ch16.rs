/*
Cryptoptals challenge 16 solution
https://www.cryptopals.com/sets/2/challenges/16
*/


use cryptoctf::symmetric::aes::{OperationMode, AES};
use cryptoctf::utils::quote_to_u8;
use cryptoctf::padding::pkcs7::{pkcs7_unpad, pkcs7_pad};

#[allow(dead_code)]
fn encrypt(data: &str) -> Vec<u8> {
    let key = "\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47".as_bytes().to_owned();
    let iv = [16u8; 16];
    let mut aes = AES::new(&key, OperationMode::CBC{ iv : iv});

    let pt = vec!["comment1=cooking%20MCs;userdata=".as_bytes().to_owned(), quote_to_u8(data), ";comment2=%20like%20a%20pound%20of%20bacon".as_bytes().to_owned()].concat();

    aes.encrypt(&pkcs7_pad(&pt, 16))
}

#[allow(dead_code)]
fn decrypt_and_tell(data: &[u8]) -> bool {
    let key =  "\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47".as_bytes();
    let iv = [16u8; 16];
    let mut aes = AES::new(&key, OperationMode::CBC{ iv : iv});
    let unpadded = pkcs7_unpad(&aes.decrypt(&data), 16).unwrap();
    let pt = String::from_utf8_lossy(&unpadded);
    println!("{}", pt);
    pt.contains("admin=true;")
}


#[cfg(test)]
mod test {

    use super::*;
    use cryptoctf::generic::xor::fixed_xor;

    #[test]
    fn set02_ch16() {
        let encrypted = encrypt("admin=true;");
        assert_eq!(decrypt_and_tell(&encrypted), false);

        /*
        comment1=cooking
        %20MCs;userdata=
        AAAAAAAAAAAAAAAA => AAAAA;admin=true
        ;comment2=%20lik
        e%20a%20pound%20
        of%20bacon
        */

        let encrypted = encrypt("AAAAAAAAAAAAAAAA");
        let mut ct2 = [0u8; 16];
        ct2.copy_from_slice(&encrypted[16..32]);
        let pt3 = b"AAAAAAAAAAAAAAAA";
        let wanted = b"AAAAA;admin=true";

        let dct3 = fixed_xor(&ct2, pt3);
        let ctm2 = fixed_xor(&dct3, wanted);
        let result = vec![&encrypted[0..16], &ctm2, &encrypted[32..encrypted.len()]].concat();

        assert_eq!(decrypt_and_tell(&result), true);
    }

}
