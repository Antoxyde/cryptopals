/*
Cryptoptals challenge 20 solution
https://www.cryptopals.com/sets/3/challenges/20
*/



#[cfg(test)]
mod test {

    use cryptoctf::rng::mt19937::MT19937;

    #[test]
    fn set03ch21() {
        let mut mt = MT19937::new(1u32);
        let res = vec![
            1791095845,
            4282876139,
            3093770124,
            4005303368,
            491263,
            550290313,
            1298508491,
            4290846341,
            630311759,
            1013994432,
        ];

        for i in  0..res.len() {
            let x = mt.rnd();
            assert_eq!(res[i], x);
        }      

    }
}
