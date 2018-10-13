/*
Cryptoptals challenge 15 solution
https://www.cryptopals.com/sets/2/challenges/15
*/

#[cfg(test)]
mod test {

    use cryptoctf::padding::pkcs7::pkcs7_oracle;

     #[test]
    fn set02_ch15() {
        assert_eq!(pkcs7_oracle(b"ICE ICE BABY\x04\x04\x04\x04", 16), true);
        assert_eq!(pkcs7_oracle(b"0123456789abcdef\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10", 16), true);
        assert_eq!(pkcs7_oracle(b"ICE ICE BABY\x05\x05\x05\x05", 16), false);
        assert_eq!(pkcs7_oracle(b"0123456789abcdef\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11\x11", 16), false);
    }
}
