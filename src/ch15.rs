/*
Cryptoptals challenge 15 solution
https://www.cryptopals.com/sets/2/challenges/15
*/

#[cfg(test)]
mod test {

    use libs::padding::pkcs7_oracle;

     #[test]
    fn ch15() {
        assert_eq!(pkcs7_oracle(b"ICE ICE BABY\x04\x04\x04\x04", 16), true);
        assert_eq!(pkcs7_oracle(b"ICE ICE BABY\x05\x05\x05\x05", 16), false);
    }
}