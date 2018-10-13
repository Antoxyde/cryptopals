/*
Cryptoptals challenge 9 solution
https://www.cryptopals.com/sets/2/challenges/9
*/

#[cfg(test)]
mod test {

    use cryptoctf::padding::pkcs7::{pkcs7_pad, pkcs7_unpad};

    #[test]
    fn set02_ch9() {
        let input = "YELLOW SUBMARINE".as_bytes().to_owned();
        let expected = b"YELLOW SUBMARINE\x04\x04\x04\x04";

        let padded = pkcs7_pad(&input, 20);
        assert_eq!(padded, expected);

        let unpadded = pkcs7_unpad(&padded, 20).unwrap();
        assert_eq!(unpadded, input);
    }
}
