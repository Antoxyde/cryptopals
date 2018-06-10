pub fn to_blocks(bytes: &[u8]) -> Vec<[u8; 16]> {
    assert!(bytes.len() % 16 == 0);

    let mut ret: Vec<[u8; 16]> = Vec::new();
    let pos: usize = bytes.len() / 16;

    for i in 0..pos {
        let mut arr = [0u8; 16];
        for j in 0..16 {
            arr[j] = bytes[i*16 + j];
        }
        ret.push(arr);
    }
    return ret;
}

pub fn is_ecb(blocks: &[[u8; 16]]) -> bool {
    for index in 0..blocks.len() {
        let occurence = blocks.into_iter().filter(|x| **x == blocks[index]).count();
        if occurence > 1 {
            return true;
        }
    }

    return false;
}
