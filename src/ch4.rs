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



#[cfg(test)]
mod test {

    use libs::xor::crack_xor;
    use libs::hex::hex_to_bytes;


    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_ch4() {

        let file = File::open("resources/4.txt");
        let mut content = String::new();

        match file {
            Ok(mut f) => {
                match f.read_to_string(&mut content) {
                    Ok(x) => x,
                    Err(e) => panic!(e),
                }
            },
            Err(e) => {
                panic!(e);
            }
        };

        let res = crack(content);

        assert_eq!("Now that the party is jumping\n", res);
    }

    fn crack(content: String) -> String {
        for try in content.split("\n") {
            if let Some(res) = crack_xor(hex_to_bytes(try)) {
                return res;
            }
        }
        return String::from("Nope");
    }
}
