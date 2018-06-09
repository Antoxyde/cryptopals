/*
Cryptoptals stage 1 challenge 7 solution
https://www.cryptopals.com/sets/1/challenges/7
*/
#[cfg(test)]
mod test {

    use libs::base64::base64_decode;
    use libs::aes::{AES, AesMode};


    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn ch7() {

        let key = "YELLOW SUBMARINE".as_bytes().to_owned();
        let mut aes = AES::new(&key, AesMode::ECB);

        let mut file = File::open("resources/7.txt").expect("File 7.txt not found.");

        let mut b64input = String::new();

        file.read_to_string(&mut b64input).expect("Error while reading 7.txt");

        let input = base64_decode(&b64input.replace("\n", ""));

        let result = String::from_utf8(aes.decrypt(&input)).unwrap();

        let expected = "I'm back and I'm ringin' the bell";

        assert!(&result.starts_with(expected));
    }
}
