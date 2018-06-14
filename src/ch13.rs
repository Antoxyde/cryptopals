/*
Cryptoptals stage 2 challenge 5 solution
https://www.cryptopals.com/sets/2/challenges/5
*/

use libs::aes::{AES, OperationMode};
use libs::padding::{pkcs7_pad, pkcs7_unpad};

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct Profile {
    email: String,
    uid: u32,
    role: String,
}

#[allow(dead_code)]
const KEY: &[u8] = b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47";


impl Profile {

    #[allow(dead_code)]
    pub fn new_from_string(data: &str) -> Self {

        //Remove all non printable bytes
        let sanitized: String =
            data.chars()
            .filter(|x| x.is_ascii_punctuation() || x.is_ascii_alphabetic() || x.is_ascii_digit())
            .collect();

        let splitted: Vec<&str> = sanitized.split("&").collect();
        assert_eq!(splitted.len(), 3);

        let mut email = String::new();
        let mut uid = 0u32;
        let mut role = String::new();

        for i in 0..3 {
            let mut key_and_value = splitted[i].split("=").collect::<Vec<_>>();
            assert_eq!(key_and_value.len(), 2);

            let k = key_and_value[0];
            let v = key_and_value[1];

            match k {
                "email" => {
                    email = String::from(v);
                },
                "uid" => {
                    uid = v.parse::<u32>().expect("Wtf");
                },
                "role" => {
                    role =  String::from(v)
                },
                _ => panic!("Invalide key"),
            }
        }

        Profile { email: email, uid: uid, role : role}
    }

    #[allow(dead_code)]
    pub fn new_from_encrypted(data: &[u8]) -> Self {
        let mut aes = AES::new(&KEY, OperationMode::ECB);

        let decrypted = String::from_utf8(pkcs7_unpad(&aes.decrypt(&data), 16).expect("Invalid padding")).expect("Utf8 error");

        return Profile::new_from_string(&decrypted);
    }

    #[allow(dead_code)]
    pub fn get_encrypted(&self) -> Vec<u8> {
        let mut aes = AES::new(&KEY, OperationMode::ECB);
        let data = format!("email={}&uid={}&role={}", self.email, self.uid, self.role);
        return aes.encrypt(&pkcs7_pad(data.as_bytes(), 16));
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        return format!("email={}&uid={}&role={}", self.email, self.uid, self.role);
    }
}

#[cfg(test)]
mod test {

    use super::Profile;

    #[test]
    fn ch13() {

        let profile = Profile::new_from_string("email=foo@bar.com&uid=10&role=user");
        let encrypted = profile.get_encrypted();
        let profile_again = Profile::new_from_encrypted(&encrypted);
        assert_eq!(profile, profile_again);

        /*
        Plan :
        email=padpapadpa
        admin\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00
        &uid=10000&role=
        osef

        =>

        email=padpapadpa
        &uid=10000&role=
        admin\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00

        I could also have used pkcs7_pad("admin", 16) to get a valid padding at the end
        */

        let malicious_email = String::from("padpapadpaadmin\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        let profile = Profile {email: malicious_email, uid: 10000, role: String::from("user") };

        let encrypted = profile.get_encrypted();

        let mut first_block =  [0u8; 16];
        let mut admin_block =  [0u8; 16];
        let mut second_block =  [0u8; 16];

        admin_block.copy_from_slice(&encrypted[16..32]);
        first_block.copy_from_slice(&encrypted[0..16]);
        second_block.copy_from_slice(&encrypted[32..48]);

        let mut crafted: Vec<u8> = Vec::new();

        for x in first_block.iter() {
            crafted.push(*x);
        }

        for x in second_block.iter() {
            crafted.push(*x);
        }

        for x in admin_block.iter() {
            crafted.push(*x);
        }

        let admin_profile = Profile::new_from_encrypted(&crafted);

        assert_eq!(admin_profile.role , "admin");
    }

}
