extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::encodings::base64::*;

    #[test]
    fn test_base64() {
        let input = String::from("CRIwqt4+szDbqkNY+I0qbDe3LQz0wiw0SuxBQtAM5TDdMbjCMD/venUDW9BL");
        let inp = input.clone();
        let dec = base64_decode(&inp);
        println!("{:x?}", dec);
        let enc = base64_encode(&dec);
        println!("{:x?}", input);
        println!("{:x?}", enc);
        assert_eq!(input, enc);

    }

}
