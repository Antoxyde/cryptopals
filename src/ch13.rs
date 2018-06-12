/*
Cryptoptals stage 2 challenge 5 solution
https://www.cryptopals.com/sets/2/challenges/5
*/

use libs::aes::{AES, OperationMode};
use libs::padding::{pkcs7_pad, pkcs7_unpad};


pub struct Profile {
    email: String,
    uid: u8,
    role: String,
}

const key: &[u8] = b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47";


impl Profile {

    pub fn new_from_string(data: &str) -> Self {
        //Remove all non printable bytes
        let sanitized: String =
            data.chars()
            .filter(|x| x.is_ascii_punctuation() || x.is_ascii_alphabetic())
            .collect();

        let splitted: Vec<&str> = sanitized.split("=").collect();
        assert_eq!(splitted.len(), 3);

        let mut email = String::new();
        let mut uid = 0u8;
        let mut role = String::new();

        for i in 0..3 {
            let (key, value) = splitted[i].split("&").collect();

            if key == "email" {
                email = value;
            }
        }

        Profile { email: email, uid: uid, role : role}
    }

    pub fn new_from_encrypted(data: &[u8]) -> Self {
        let aes = AES::new(&key, OperationMode::ECB);

        let decrypted = String::from_utf8(pkcs7_unpad(&aes.decrypt(&data), 16).expect("Invalid padding")).expect("Utf8 error");

        return Profile::new_from_string(&decrypted);
    }

    pub fn encrypt(&self) -> Vec<u8> {
        let aes = AES::new(&key, OperationMode::ECB);
        let data = format!("email={}&uid={}&role={}", self.email, self.uid, self.role);
        return pkcs7_pad(&aes.encrypt(&data.as_bytes()), 16);
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn ch13() {



    }

}
