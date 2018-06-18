

#[cfg(test)]
mod test {
    use libs::base64::base64_decode;
    use libs::aes::{OperationMode, AES};

    #[test]
    fn ch16() {

        let key =  b"\x61\x2b\x12\x6c\x32\x39\x69\x4d\x48\x16\x64\x4e\x78\x54\x71\x47";
        let iv = key.rev();

        let aes = Aes::new(&key, OperationMode::CBC{ iv = iv});

        let enc_b1 = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f";
        let enc_b2 = 
    }
    
}   
