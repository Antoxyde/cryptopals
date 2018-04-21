pub mod utils;

use utils::hex::hex_to_bytes;
use utils::base64::base64_encode;

fn main() {
    let hexstring = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let s = base64_encode(hex_to_bytes(hexstring));
    println!("{}", s);
}
