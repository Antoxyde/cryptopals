extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::symmetric::rc4::RC4;
    use cryptoctf::utils::hex_to_bytes;

    #[test]
    fn test_rc4() {

        assert_eq!(RC4::new(b"Key").encrypt(b"Plaintext"), hex_to_bytes("BBF316E8D940AF0AD3"));
        assert_eq!(RC4::new(b"Wiki").encrypt(b"pedia"), hex_to_bytes("1021BF0420"));
        assert_eq!(RC4::new(b"Secret").encrypt(b"Attack at dawn"), hex_to_bytes("45A01F645FC35B383552544B9BF5"));

    }
}
