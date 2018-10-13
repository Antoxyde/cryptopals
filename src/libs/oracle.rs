use libs::aes_utils::{is_ecb, to_blocks};
use libs::padding::pkcs7_unpad;

pub fn oracle_ecb_cbc(data: &[u8]) -> &str {
    let blocks = to_blocks(data);
    match is_ecb(&blocks) {
        true => "ECB",
        false => "CBC",
    }
}

pub fn oracle_cbc_padding(data: &[u8]) -> bool {
    match pkcs7_unpad(&data, 16) {
        Err(_) => false,
        Ok(_) => true,
    }
}
