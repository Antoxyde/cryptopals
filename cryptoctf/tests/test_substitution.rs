extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::classical::substitution::Substitution;

    #[test]
    fn test_substitution_encrypt() {
        let ps = Substitution::new(b"ZYXWVUTSRQPONMLKJIHGFEDCBA", b"ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        assert_eq!(ps.encrypt(b"ABCD").unwrap(), b"ZYXW") ;
    }

    #[test]
    fn test_substitution_decrypt() {
        let ps = Substitution::new(b"ZYXWVUTSRQPONMLKJIHGFEDCBA", b"ABCDEFGHIJKLMNOPQRSTUVWXYZ").unwrap();
        assert_eq!(ps.decrypt(b"ZYXW").unwrap() , b"ABCD");

    }


}
