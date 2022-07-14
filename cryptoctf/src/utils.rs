use itertools::Itertools;

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
    for &c in s {
        if (c as i32 >= 65 && c as i32 <= 90) || (c as i32 >= 97 && c as i32 <= 122)
            || c as char == ' '
        {
            count += 1;
        }
    }

    count as f32 / s.len() as f32
}

pub fn hamming_distance(s1: String, s2: String) -> i32 {
    // Compute the hamming distance between the to given strings,bytes by bytes
    let mut total = 0i32;
    for pair in s1.chars().zip(s2.chars()) {
        total += (pair.0 as u8 ^ pair.1 as u8).count_ones() as i32;
    }

    total
}

pub fn quote_to_u8(data: &str) -> Vec<u8> {
    data.replace(";", "%3B").replace("=", "%3D").into_bytes()
}

pub fn hex_to_bytes(hexstring: &str) -> Vec<u8> {
    let mut byte_vec: Vec<u8> = Vec::new();

    for char in m_split(hexstring, 2) {
        byte_vec.push(u8::from_str_radix(char, 16).unwrap());
    }

    byte_vec
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hexstring = String::new();
    for &b in bytes {
        hexstring.push_str(&format!("{:02x}", b as u32));
    }
    hexstring
}

pub fn print_state_hex(state: &[u8; 16]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{:x} ", state[4 * j + i]);
        }
        println!();
    }
    println!();
}

pub fn pack_with_count(m: &[u64]) -> Vec<(u64, usize)> {
    let n = Vec::from(m);
    let n2 = n.clone();
    let mut result = Vec::new();

    for v in n.into_iter().unique() {
        let c = n2.iter().filter(|x| **x == v).count();
        result.push((v, c));
    }

    result
}

// from https://gist.github.com/kylewlacy/115965b40e02a3325558

pub fn partial_cartesian<T: Clone>(a: Vec<Vec<T>>, b: &[T]) -> Vec<Vec<T>> {
    a.into_iter().flat_map(|xs| {
        b.iter().cloned().map(|y| {
            let mut vec = xs.clone();
            vec.push(y);
            vec
        }).collect::<Vec<_>>()
    }).collect()
}


pub fn cartesian_product<T: Clone>(lists: &[&[T]]) -> Vec<Vec<T>> {
    match lists.split_first() {
        Some((first, rest)) => {
            let init: Vec<Vec<T>> = first.iter().cloned().map(|n| vec![n]).collect();

            rest.iter().cloned().fold(init, |vec, list| {
                partial_cartesian(vec, list)
            })
        },
        None => {
            vec![]
        }
    }
}

pub fn to_blocks(bytes: &[u8], blocksize: usize) -> Vec<Vec<u8>> {
    assert!(bytes.len() % blocksize == 0);

    let mut ret: Vec<Vec<u8>> = Vec::new();
    let pos: usize = bytes.len() / blocksize;

    for i in 0..pos {
        let mut arr = Vec::new();
        for j in 0..blocksize {
            arr.push(bytes[i*blocksize + j]);
        }
        ret.push(arr);
    }
    ret
}



