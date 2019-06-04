/*
Cryptoptals challenge 20 solution
https://www.cryptopals.com/sets/3/challenges/20
*/



#[cfg(test)]
mod test {

    use cryptoctf::encodings::base64::base64_decode;

    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn set03_ch20() {
        let mut file = File::open("resources/20.txt").expect("File 20.txt not found.");
        let mut b64input = String::new();
        file.read_to_string(&mut b64input).expect("Error while reading 20.txt");
        
        let mut cts = Vec::new();
        
        for line in b64input.lines() {
            cts.push(base64_decode(&line));
        }
        
        //TODO
        println!("{:?}", cts);        
    }
}
