/*
Cryptoptals challenge 1 solution
https://www.cryptopals.com/sets/1/challenges/1
 */



#[cfg(test)]
mod test {
    use cryptoctf::utils::hex_to_bytes;
    use cryptoctf::encodings::base64::base64_encode;

    #[test]
    fn set01_ch1() {
        let hexstring = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let s = base64_encode(&hex_to_bytes(hexstring));

        assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", s);
    }
}
