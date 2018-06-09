/*
Cryptoptals stage 1 challenge 8 solution
https://www.cryptopals.com/sets/1/challenges/8
*/



#[cfg(test)]
mod test {

    use libs::hex::hex_to_bytes;
    use libs::aes_utils::{to_blocks, is_ecb};

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn ch8() {


        let mut file = File::open("resources/8.txt").expect("File 8.txt not found.");
        let mut input = String::new();
        file.read_to_string(&mut input).expect("Error while reading 8.txt");

        let mut ecb_line: Option<&str> = None;

        for line in input.split("\n") {
            let blocks = to_blocks(&hex_to_bytes(&line));
            if is_ecb(&blocks) {
                ecb_line = Some(line);
                break;
            }
        }

        let expected = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        assert_eq!(ecb_line.unwrap(), expected);


    }
}
