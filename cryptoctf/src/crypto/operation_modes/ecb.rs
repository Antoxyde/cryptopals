pub fn is_ecb(blocks: &[[u8; 16]]) -> bool {

    for &b in blocks {
        let occurence = blocks.iter().filter(|x| **x == b).count();
        if occurence > 1 {
            return true;
        }
    }

    false
}
