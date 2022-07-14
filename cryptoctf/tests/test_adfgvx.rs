extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::classical::adfgvx::Adfgvx;

    #[test]
    fn test_adfgvx_encrypt() {
        let adfgvx = Adfgvx::new(b"c1ofwjymt5b4i7a28sp30qhxkeul6dvrgzn9", b"MARCEL").unwrap();
        assert_eq!(adfgvx.encrypt(b"objectifarras15h28"), Vec::from("FDADXVVAGDDGADFFDFXFFFGVAVFXFGDAAXAF".as_bytes()));

        let adf = Adfgvx::new(b"na1c3h8tb2ome5wrpd4f6g7i9j0klqsuvxyz", b"PRIVACY").unwrap();
        assert_eq!(adf.encrypt(b"attackat1200am"), Vec::from("DGDDDAGDDGAFADDFDADVDVFAADVX".as_bytes()));

    }

    
    #[test]
    fn test_adfgvx_decrypt() {


         let adfgvx = Adfgvx::new(b"c1ofwjymt5b4i7a28sp30qhxkeul6dvrgzn9",b"MARCEL").unwrap();

         assert_eq!(adfgvx.decrypt(b"FDADXVVAGDDGADFFDFXFFFGVAVFXFGDAAXAF"), Vec::from("objectifarras15h28".as_bytes()));
    }
    

}
