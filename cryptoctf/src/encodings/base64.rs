use utils::m_split;

pub fn base64_encode(data: &[u8]) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    let mut encoded = String::new();

    let max = data.len() - data.len() % 3;
    let mut i = 0;

    while i < max {
        let mut total = (data[i] as u64) << 16;
        total += (data[i + 1] as u64) << 8;
        total += data[i + 2] as u64;

        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push(charset[((total >> 6) & 63) as usize] as char);
        encoded.push(charset[(total & 63) as usize] as char);

        i += 3;
    }

    if data.len() % 3 == 2 {
        let mut total = (data[data.len() - 2] as u64) << 16;
        total += (data[data.len() - 1] as u64) << 8;
        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push(charset[((total >> 6) & 63) as usize] as char);
        encoded.push('=');
    } else if data.len() % 3 == 1 {
        let total = (data[data.len() - 1] as u64) << 16;
        encoded.push(charset[((total >> 18) & 63) as usize] as char);
        encoded.push(charset[((total >> 12) & 63) as usize] as char);
        encoded.push('=');
        encoded.push('=');
    }

    encoded
}

pub fn base64_decode(data: &str) -> Vec<u8> {
    let mut decoded: Vec<u8> = Vec::new();
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    if data.len() % 4 != 0 {
        panic!("Invalid base64 input.");
    }

    for sl in m_split(&data.replace("=", "A"), 4) {

        let ch1 = sl.chars().next().unwrap();
        let ch2 = sl.chars().nth(1).unwrap();
        let ch3 = sl.chars().nth(2).unwrap();
        let ch4 = sl.chars().nth(3).unwrap();

        let enc1: u64 = (charset.find(ch1).unwrap() as u64) << 18;
        let enc2: u64 = (charset.find(ch2).unwrap() as u64) << 12;
        let enc3: u64 = (charset.find(ch3).unwrap() as u64) << 6;
        let enc4: u64 = charset.find(ch4).unwrap() as u64;

        let total = enc1 + enc2 + enc3 + enc4;

        let dec1 = ((total >> 16) & 0xff) as u8;
        let dec2 = ((total >> 8) & 0xff) as u8;
        let dec3 = (total & 0xff) as u8;

        decoded.push(dec1);
        decoded.push(dec2);
        decoded.push(dec3);
    }

    while decoded[decoded.len() - 1 ] == 0 {
        decoded.pop();
    }

    decoded
}


