extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::symmetric::aes_dfa;
    use cryptoctf::symmetric::aes_core::*;

    #[test]
    fn test_aes_dfa() {

		let k = key_expansion(&[0x2a, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x52, 0x09, 0xcf, 0x4f,0x3c,], 4, 10);

        let mut good = [0; 16];
        cipher(&k, &mut good, 10, aes_dfa::AesFault{index:0, value:0, round:9});

        let mut faulteds = Vec::new();

        // Collecting faulty ciphertexts

        for i in 1..4 {
            let mut faulted =  [0; 16];
            cipher(&k, &mut faulted, 10, aes_dfa::AesFault{index:0, value:i, round:9});
            faulteds.push(faulted.clone());
        }

        for i in 1..4 {
            let mut faulted =  [0; 16];
            cipher(&k, &mut faulted, 10, aes_dfa::AesFault{index:4, value:i, round:9});
            faulteds.push(faulted.clone());
        }


        for i in 1..4 {
            let mut faulted =  [0; 16];
            cipher(&k, &mut faulted, 10, aes_dfa::AesFault{index:8, value:i, round:9});
            faulteds.push(faulted.clone());
        }

        for i in 1..4 {
            let mut faulted =  [0; 16];
            cipher(&k, &mut faulted, 10, aes_dfa::AesFault{index:12, value:i, round:9});
            faulteds.push(faulted.clone());
        }

        let k_recovered = aes_dfa::recover_aes_key_r9_injection([0; 16], good, &faulteds).unwrap();

        assert_eq!(k, key_expansion(&k_recovered, 4, 10));
	}


}
