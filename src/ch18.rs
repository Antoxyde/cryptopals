/*
Cryptoptals challenge 18 solution
https://www.cryptopals.com/sets/3/challenges/18
*/

use cryptoctf::symmetric::aes::{OperationMode, AES};
use cryptoctf::encodings::base64::base64_decode;


#[cfg(test)]
mod test {

    use super::*;
    
   #[test]
   fn set03_ch18() {
        let ciphertext = base64_decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==");
        let key = "YELLOW SUBMARINE".as_bytes(); 
        let mut aes = AES::new(&key, OperationMode::CTR{ nonce: [0u8;8]});

        assert_eq!("Yo, VIP Let's kick it Ice, Ice, baby Ice, Ice, baby ", String::from_utf8_lossy(&aes.decrypt(&ciphertext)));
    }
}
