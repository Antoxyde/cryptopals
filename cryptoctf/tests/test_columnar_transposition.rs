extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::classical::columnar_transposition::ColumnarTransposition;

    #[test]
    fn test_columnar_transposition_encrypt() {

        let ct = ColumnarTransposition::new(b"acb").unwrap();
        assert_eq!(ct.encrypt(b"ABCDEFGH"), b"ADGCFBEH") ;

        let ct2 = ColumnarTransposition::new(b"BDAC").unwrap();
        assert_eq!(ct2.encrypt(b"MYSUPERDANKTEST"), b"SRKTMPAEUDTYENS") ;


    }

    #[test]
    fn test_columnar_transposition_decrypt() {
        //let ct = ColumnarTransposition::new(b"acb").unwrap();
        //assert_eq!(ct.decrypt(b"ADGCFBEH"), b"ABCDEFGH") ;

        //let ct2 = ColumnarTransposition::new(b"BDAC").unwrap();
        //assert_eq!(ct2.decrypt(b"SRKTMPAEUDTYENS"), b"MYSUPERDANKTEST") ;
    }


}
