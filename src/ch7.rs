/*
Cryptoptals stage 1 challenge 7 solution
https://www.cryptopals.com/sets/1/challenges/7
*/
#[cfg(test)]
mod test {
use libs::aes::{test};

    #[test]
    fn test_ch7() {
        test();

        /*
        let key = "YELLOW SUBMARINE".as_bytes().to_owned();
        let inb = "Attack at Dawn!!".as_bytes().to_owned();
        let mut outb: Vec<u8> = Vec::new();
        aes_encrypt(key, &inb, &mut outb);
        println!("Encrypted block : {:?}", outb);
        */
    }
}
