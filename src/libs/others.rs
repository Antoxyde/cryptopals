pub fn m_split(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}


pub fn count_letter_and_spaces(s: &[u8]) -> f32 {
    let mut count = 0i32;
    for i in 0..s.len() {
        if (s[i] as i32 >= 65 && s[i] as i32 <= 90) || (s[i] as i32 >= 97 && s[i] as i32 <= 122)
            || s[i] as char == ' '
        {
            count += 1;
        }
    }

    count as f32 / s.len() as f32
}

pub fn hamming_distance(s1: String, s2: String) -> i32 {
    let mut total = 0i32;
    for pair in s1.chars().zip(s2.chars()) {
        total += (pair.0 as u8 ^ pair.1 as u8).count_ones() as i32;
    }
    total
}

pub fn quote_to_u8(data: &str) -> Vec<u8> {
    data.replace(";", "%3B").replace("=", "%3D").into_bytes()
}