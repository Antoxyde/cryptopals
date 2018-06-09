/*
Cryptoptals stage 1 challenge 3 solution
https://www.cryptopals.com/sets/1/challenges/3
*/
#[cfg(test)]
mod test {
    use libs::xor::crack_xor;
    use libs::hex::hex_to_bytes;

    #[test]
    fn ch3() {
        let s = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let pr = crack_xor(s);
        assert_eq!("Cooking MC's like a pound of bacon", pr.unwrap());
    }
}
