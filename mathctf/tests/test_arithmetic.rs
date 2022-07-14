extern crate mathctf;
extern crate num_bigint;

#[cfg(test)]
mod test {

    use mathctf::arithmetic::*;
    use num_bigint::{ToBigInt,BigInt, BigUint,ToBigUint};

    #[test]
    fn test_big_egcd() {
        let a = BigInt::parse_bytes(b"857152454024", 10).unwrap();
        let b = BigInt::parse_bytes(b"718433359630", 10).unwrap();
        let u = BigInt::parse_bytes(b"20656833503", 10).unwrap();
        let v = BigInt::parse_bytes(b"24645369389", 10).unwrap();
        let two = 02.to_bigint().unwrap();

        assert_eq!(big_egcd(&a,&b), (two,u, -v));

    }
    
    #[test]
    fn test_inv_mod() {
        let b = BigUint::parse_bytes(b"1361129467683753853853498429727072845819", 10).unwrap(); // 2**130 - 5
        let a = 224815.to_biguint().unwrap();
        let res = BigUint::parse_bytes(b"1117413943398711362769635138620013306484", 10).unwrap();
        assert_eq!(inv_mod(&a,&b).unwrap(), res);
    }

    #[test]
    fn test_chinese_remainder() {
        let modulis = vec![3.to_biguint().unwrap(),4.to_biguint().unwrap(),5.to_biguint().unwrap()];
        let residues = vec![0.to_biguint().unwrap(),3.to_biguint().unwrap(),4.to_biguint().unwrap()];
        assert_eq!(chinese_remainder(&modulis, &residues).unwrap(), 39.to_biguint().unwrap());


    }

}
