/*
Cryptoptals  challenge 4 solution
https://www.cryptopals.com/sets/1/challenges/4
*/

use cryptoctf::generic::xor::crack_xor;
use cryptoctf::utils::hex_to_bytes;

#[allow(dead_code)]
fn crack(content: String) -> String {
    for try in content.split("\n") {
        if let Some(res) = crack_xor(&hex_to_bytes(try)) {
            return res;
        }
    }
    return String::from("Nope");
}


#[cfg(test)]
mod test {

    use std::fs::File;
    use std::io::prelude::*;

    use super::*;

    #[test]
    fn set01_ch4() {

        let mut file = File::open("resources/4.txt").expect("File 4.txt not found.");
        let mut content = String::new();

        file.read_to_string(&mut content).expect("Error while reading 4.txt");

        let res = crack(content);

        assert_eq!("Now that the party is jumping\n", res);
    }


}
