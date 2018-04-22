use include::others::m_split;

pub fn hex_to_bytes(hexstring: &str) -> Vec<u8> {
    let mut byte_vec: Vec<u8> = Vec::new();

    for char in m_split(hexstring, 2) {
        byte_vec.push(u8::from_str_radix(char, 16).unwrap());
    }

    byte_vec
}

pub fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut hexstring = String::new();
    for i in 0..bytes.len() {
        hexstring.push_str(&format!("{:02x}", bytes[i] as u32));
    }
    hexstring
}
