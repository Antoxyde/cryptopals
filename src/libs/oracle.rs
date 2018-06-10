use libs::aes_utils::{is_ecb, to_blocks};

pub fn oracle_ecb_cbc(data: &[u8]) -> &str {
    let blocks = to_blocks(data);
    match is_ecb(&blocks) {
        true => "ECB",
        false => "CBC",
    }
}
