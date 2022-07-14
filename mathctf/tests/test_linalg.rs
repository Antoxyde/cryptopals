extern crate mathctf;
extern crate bigdecimal;

#[cfg(test)]
mod test {
    use mathctf::{linalgs, primitives};
    use bigdecimal::BigDecimal;

    #[test]
    fn test_base() {
        let m1 = primitives::Matrix::diag(&[1.0.into(),2.0.into(),3.0.into()]);
        let m2 = m1.clone();
        let r = m1 * m2;
        println!("R : {}", r);
    }

    #[test]
    fn test_gram_schmidt() {
        let b = vec![
            primitives::Vector  { data: vec![3.0.into(), 1.0.into()]},
            primitives::Vector  { data: vec![2.0.into(), 2.0.into()]},
            ];

        let r = vec![
            primitives::Vector  { data: vec![3.0.into(), 1.0.into()]},
            primitives::Vector  { data: vec![BigDecimal::from(-0.4), BigDecimal::from(6.0/5.0)]},
            ];

        assert_eq!(linalgs::classical_gram_schmidt(&b, false), r);
    }

    #[test]
    fn test_lll() {
        let input_basis = vec![
            primitives::Vector { data:vec![1.into(), 1.into(), 1.into()]},
            primitives::Vector { data:vec![BigDecimal::from(-1), 0.into(), 2.into()]},
            primitives::Vector { data:vec![3.into(), 5.into(), 6.into()]},
        ];

        println!("Result : {:?}", linalgs::lll(&input_basis, &(3.0/4.0).into()));
    }

    #[test]
    fn test_coppersmith() {
        
    }
}
