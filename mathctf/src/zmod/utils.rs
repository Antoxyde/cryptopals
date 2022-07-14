use num_integer::binomial;

use crate::zmod::poly::ZModPoly;
use crate::zmod::scalar::ZModScalar;
use num_traits::Pow;

// Expands (x + b)^n
pub fn expands_binomial(b: &ZModScalar, n: u64) -> ZModPoly {

    let mut coeffs = Vec::new();
    for k in 0..=n {
        let a = binomial(n, k) * b.clone().pow(n - k).value;
        coeffs.push(b.ring.scalar(&a));
    }

    ZModPoly { coeffs , ring: b.ring.clone() }
}
