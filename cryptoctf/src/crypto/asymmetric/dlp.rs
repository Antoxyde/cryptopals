
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;
use mathctf::arithmetic::inv_mod;

use num_iter;

// Shank's algorithm aka Baby Step giant Step
// return x such that g^x = h in Z/nZ

#[allow(clippy::many_single_char_names)]
pub fn baby_step_giant_step(g: &BigUint, h: &BigUint, n: &BigUint) -> Option<BigUint> {

    let mut babys: HashMap<BigUint, BigUint> = HashMap::new();

    let m  = n.clone().sqrt();

    for j in  num_iter::range_inclusive(One::one(),m.clone()) {
        babys.insert(g.modpow(&j, n), j.clone());
    };

    let inv = inv_mod(&g.modpow(&m, n), n).expect("Inverse modulo doesnt exists, n isnt prime?");

    let mut y = h.clone();

    for i in num_iter::range_inclusive(Zero::zero(),m.clone()) {

        if babys.contains_key(&y) {
            return  Some(i * &m + babys.get(&y).unwrap());
        }

        y = (&y * &inv) % n;

    }

    None
}

