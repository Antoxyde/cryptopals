use operation_modes::ecb::is_ecb;
use symmetric::aes_utils::to_aes_blocks;

use padding::pkcs7::pkcs7_unpad;

pub fn oracle_ecb_cbc(data: &[u8]) -> &str {
    let blocks = to_aes_blocks(data);
    if is_ecb(&blocks) {
        "ECB"
    } else {
        "CBC"
    }

}

pub fn oracle_cbc_padding(data: &[u8]) -> bool {
    pkcs7_unpad(&data, 16).is_ok()
}
