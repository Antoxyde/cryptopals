/*
Cryptoptals stage 2 challenge 2 solution
https://www.cryptopals.com/sets/2/challenges/2
*/

#[cfg(test)]
mod test {

    use libs::base64::base64_decode;
    use libs::aes::{AES, OperationMode};

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn ch10() {
        let key = "YELLOW SUBMARINE".as_bytes().to_owned();
        let mode = OperationMode::CBC {iv: [0u8; 16]};
        let mut aes = AES::new(&key, mode);

        let mut file = File::open("resources/10.txt").expect("File 10.txt not found.");
        let mut b64input = String::new();
        file.read_to_string(&mut b64input).expect("Error while reading 7.txt");
        let input = base64_decode(&b64input.replace("\n", ""));

        let result = String::from_utf8(aes.decrypt(&input)).unwrap();
        let expected = "I'm back and I'm ringin' the bell";

        assert!(&result.starts_with(expected));
    }
}
