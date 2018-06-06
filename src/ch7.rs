/*
Cryptoptals stage 1 challenge 7 solution
https://www.cryptopals.com/sets/1/challenges/7
*/
#[cfg(test)]
mod test {
    use libs::aes::AES;

    #[test]
    fn ch7() {

        let key = "YELLOW SUBMARINE".as_bytes().to_owned();
        let mut aes = AES::new(&key);
        let inb = [0u8; 16];

        for l in 0.."Attack at Dawn!!".len() {

        }

        let out = aes.encrypt(&inb);
        //aes_encrypt(key, &inb, &mut outb);
        println!("Encrypted block : {:?}", out);
    }
}
