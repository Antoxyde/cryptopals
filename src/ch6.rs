/*
Cryptoptals challenge 6 solution
https://www.cryptopals.com/sets/1/challenges/6
*/


#[cfg(test)]
mod test {

    use std::fs::File;
    use std::io::prelude::*;

    use libs::base64::base64_decode;
    use libs::others::{hamming_distance, m_split};
    use libs::xor::{crack_xor_key, key_cycling_xor};

    #[test]
    fn ch6() {

        assert_eq!(hamming_distance("this is a test".to_string(), "wokka wokka!!!".to_string()), 37);

        let mut file = File::open("resources/6.txt").expect("File 6.txt not found.");
        let mut encoded_content = String::new();

        file.read_to_string(&mut encoded_content).expect("Error while reading 6.txt");

        let content: String =
            String::from_utf8(base64_decode(&encoded_content.replace("\n", ""))).unwrap();

        let mut keysize_guess: Vec<(i32, i32)> = Vec::new();

        for guess in 2..40 as usize {
            //Could be optimized ? it take several seconds ..
            let splitted = m_split(&content, guess); //make blocks of guess size;
            let mut counter = 0;
            let mut hd = 0;
            for x in &splitted {
                // loop over thoses blocks and compute their hamming distance
                for y in &splitted {
                    if x != y {
                        //compute the hamming distance of the same blocks would be idiot ?
                        hd += hamming_distance(x.to_string(), y.to_string());
                        counter += 1;
                    }
                }
            }

            keysize_guess.push((guess as i32, (hd / counter) / guess as i32));
        }

        keysize_guess.sort_by(|a, b| a.1.cmp(&b.1)); //Sort by lower hamming distance
        let keysize: i32 = keysize_guess[0].0; //Take the lowest one

        let mut zipped: Vec<String> = Vec::new();

        for _ in 0..keysize {
            //init a vector of string
            zipped.push(String::new());
        }

        for (c, item) in content.chars().enumerate() {
            //zip our strings between themselves
            zipped[c % keysize as usize].push(item);
        }

        let mut key: Vec<u8> = Vec::new();

        for z in zipped {
            //crack each zip one by one (which correspond to each key char)
            key.push(crack_xor_key(z.into_bytes()).unwrap())
        }

        let u_content = content.clone().into_bytes();

        let _plaintext = String::from_utf8(key_cycling_xor(&u_content, &key)).unwrap(); //Decrypt the content with the key we jsut found

        assert_eq!("Terminator X: Bring the noise".as_bytes().to_owned() , key);
    }
}
