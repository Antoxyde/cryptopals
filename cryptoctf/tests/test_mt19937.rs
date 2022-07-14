extern crate cryptoctf;

#[cfg(test)]
mod test {

    use cryptoctf::rng::mt19937::MT19937;

    #[test]
    fn test_mt19937() {

        let mut mt = MT19937::new(1u32);
        let res = vec![
            1_791_095_845,
            4_282_876_139,
            3_093_770_124,
            4_005_303_368,
            491_263,
            550_290_313,
            1_298_508_491,
            4_290_846_341,
            630_311_759,
            1_013_994_432,
        ];

        for i in  0..res.len() {
            let x = mt.rnd();
            assert_eq!(res[i], x);
        }

    }
}
