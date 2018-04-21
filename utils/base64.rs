use utils::others::m_split;

pub fn base64_encode(bytes: Vec<u8>) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut encoded = String::new();

    let max = bytes.len() - bytes.len() % 3;
    let mut i = 0;

    while i < max {
        let mut total = (bytes[i] as u64) << 16;
        total += (bytes[i + 1] as u64) << 8;
        total += bytes[i + 2] as u64;

        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push(charset[((total >> 6) & 63) as usize] as char);
        encoded.push(charset[(total & 63) as usize] as char);

        i += 3;
    }

    if bytes.len() % 3 == 2 {
        let mut total = (bytes[bytes.len() - 2] as u64) << 16;
        total += (bytes[bytes.len() - 1] as u64) << 8;
        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push(charset[((total >> 6) & 63) as usize] as char);
        encoded.push('=');
    } else if bytes.len() % 3 == 1 {
        let total = (bytes[bytes.len() - 1] as u64) << 16;
        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push('=');
        encoded.push('=');
    }

    encoded
}

pub fn base64_decode(data: String) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();

    if data.len() % 4 != 0 {
        panic!("Invalid base64 input.");
    }

    for sl in m_split(&data.replace("=", "A"), 4) {
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut total = (charset.find(sl.chars().nth(0).unwrap()).unwrap() as u64) << 18;
        total += (charset.find(sl.chars().nth(1).unwrap()).unwrap() as u64) << 12;
        total += (charset.find(sl.chars().nth(2).unwrap()).unwrap() as u64) << 6;
        total += charset.find(sl.chars().nth(3).unwrap()).unwrap() as u64;

        decoded.push(((total >> 16) & 127) as u8);
        decoded.push(((total >> 8) & 127) as u8);
        decoded.push((total & 127) as u8);
    }

    decoded
}
