/*
Cryptoptals challenge 2 solution
https://www.cryptopals.com/sets/1/challenges/2
*/

#[cfg(test)]
mod test {

    use cryptoctf::utils::{bytes_to_hex, hex_to_bytes};
    use cryptoctf::generic::xor::fixed_xor;

    #[test]
    fn set01_ch2() {
        let s1 = hex_to_bytes("1c0111001f010100061a024b53535009181c");
        let s2 = hex_to_bytes("686974207468652062756c6c277320657965");
        let s3 = bytes_to_hex(&fixed_xor(&s1, &s2));
        assert_eq!("746865206b696420646f6e277420706c6179", s3);
    }
}
