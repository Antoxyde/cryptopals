use include::others::count_letter_and_spaces;

pub fn fixed_xor(s1: Vec<u8>, s2: Vec<u8>) -> Vec<u8> {
    assert_eq!(s1.len(), s2.len());
    let mut res = Vec::new();

    for i in 0..s1.len() {
        res.push(s1[i] ^ s2[i]);
    }

    res
}

pub fn single_byte_xor(bytes: &[u8], k: u8) -> Vec<u8> {
    let mut xored: Vec<u8> = Vec::new();

    for i in 0..bytes.len() {
        xored.push(bytes[i] ^ k);
    }

    xored
}

pub fn crack_xor(bytes_ct: Vec<u8>) -> Option<String> {
    for i in 0..120 {
        let res = single_byte_xor(&bytes_ct, i);

        let ratio = count_letter_and_spaces(&res);
        if ratio > 0.80 {
            return Some(String::from_utf8(res).unwrap());
        }
    }

    None
}

pub fn crack_xor_key(bytes_ct: Vec<u8>) -> Option<u8> {
    for i in 0..127 {
        let res = single_byte_xor(&bytes_ct, i);

        let ratio = count_letter_and_spaces(&res);
        if ratio > 0.80 {
            return Some(i as u8);
        }
    }

    None
}

pub fn key_cycling_xor(plain: &[u8], key: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for i in 0..plain.len() {
        result.push(plain[i] ^ key[i % key.len()]);
    }

    result
}
