extern crate cryptoctf;
extern crate num_bigint;

#[cfg(test)]
mod test {

    use cryptoctf::asymmetric::dlp;
    use num_bigint::BigUint;

    #[test]
    fn test_bsgs() {

        let h = BigUint::parse_bytes(b"472701959", 10).unwrap();
        let g = BigUint::parse_bytes(b"56478764", 10).unwrap();
        let n = BigUint::parse_bytes(b"2134568463", 10).unwrap();

        let x = dlp::baby_step_giant_step(&g, &h , &n);

        assert_eq!(x.unwrap(), BigUint::parse_bytes(b"216623701", 10).unwrap());
    }
}
