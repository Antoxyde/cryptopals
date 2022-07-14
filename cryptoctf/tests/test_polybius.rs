extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::classical::polybius_square::PolybiusSquare;

    #[test]
    fn test_polybius_encrypt() {
        let ps = PolybiusSquare::new(b"12345", b"ABCDEFGHIKLMNOPQRSTUVWXYZ").unwrap();
        assert_eq!(ps.encrypt(b"ABCDEFGHIKLMNOPQRSTUVWXYZ"), Vec::from("11121314152122232425313233343541424344455152535455".as_bytes()));
    }

    #[test]
    fn test_polybius_decrypt() {
         let ps = PolybiusSquare::new(b"12345", b"ABCDEFGHIKLMNOPQRSTUVWXYZ").unwrap();
         assert_eq!(ps.decrypt(b"11121314152122232425313233343541424344455152535455") , Vec::from("ABCDEFGHIKLMNOPQRSTUVWXYZ".as_bytes()));
    }


}
