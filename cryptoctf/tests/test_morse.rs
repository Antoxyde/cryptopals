extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::classical::morse::Morse;

    #[test]
    fn test_morse_encode() {
        let morse = Morse::new();
        println!("{:?}",morse.encode(&vec!["H", "E", "L", "L", "O"]).unwrap());

        assert_eq!(morse.encode(&vec!["H", "E", "L", "L", "O"]).unwrap(), vec!["....", ".", ".-..",".-..", "---"]) ;
        assert_eq!(morse.encode_str("HELLO").unwrap(), ".... . .-.. .-.. ---") ;

    }

    #[test]
    fn test_morse_decode() {
        let morse = Morse::new(); 
        assert_eq!(morse.decode(&vec!["....", ".", ".-..", ".-..", "---"]).expect("decode"), vec!["H", "E", "L", "L", "O"]) ;
        assert_eq!(morse.decode_str(".... . .-.. .-.. ---").expect("decode_str"), "HELLO") ;

    }


}
