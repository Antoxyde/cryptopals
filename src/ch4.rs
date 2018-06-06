/*
Cryptoptals stage 1 challenge 4 solution
https://www.cryptopals.com/sets/1/challenges/4

Simply try every possible char to break xor on every hexstring of the file and if their is more than 90% of ascii char and space , it's probably the valid string.

Output :
[antoxyde@anarchy-fixe:Projets/Cryptopals]$ rustc ch4.rs
warning: unused `std::result::Result` which must be used
  --> ch4.rs:20:5
   |
20 |     file.unwrap().read_to_string(&mut content);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default

[antoxyde@anarchy-fixe:Projets/Cryptopals]$ ./ch4
Now that the party is jumping


*/
use libs::xor::crack_xor;
use libs::hex::hex_to_bytes;

#[allow(dead_code)]
fn crack(content: String) -> String {
    for try in content.split("\n") {
        if let Some(res) = crack_xor(hex_to_bytes(try)) {
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
    fn ch4() {

        let mut file = File::open("resources/4.txt").expect("File 4.txt not found.");
        let mut content = String::new();

        file.read_to_string(&mut content).expect("Error while reading 4.txt");

        let res = crack(content);

        assert_eq!("Now that the party is jumping\n", res);
    }


}
