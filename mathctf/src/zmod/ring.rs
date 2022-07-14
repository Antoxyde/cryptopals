use crate::zmod::scalar::ZModScalar;
use crate::zmod::poly::ZModPoly;
use num_bigint::BigUint;
use std::fmt;
use num_traits::{Zero, One};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZModRing {
    pub modulus: BigUint,
}

impl fmt::Display for ZModRing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ring of elements modulo {} + ", self.modulus)
    }
}

impl ZModRing {

    pub fn new(modulus: &BigUint) -> Self {
        ZModRing { modulus: modulus.clone() }
    }

    pub fn scalar(&self, n: &BigUint) -> ZModScalar {
        ZModScalar { value: n % &self.modulus ,ring: self.clone() }
    }

    pub fn zero(&self) -> ZModScalar {
        self.scalar(&BigUint::zero())
    }

    pub fn one(&self) -> ZModScalar {
        self.scalar(&BigUint::one())
    }

    pub fn x(&self) -> ZModPoly {
        ZModPoly { coeffs: vec![self.zero(), self.one()], ring: self.clone() }
    }

    pub fn poly_from_biguint_vec(&self, coeffs: &[BigUint]) -> ZModPoly {

        let mut v = Vec::new();
        for coeff in coeffs.iter() {
            v.push(self.scalar(coeff));
        }
        ZModPoly { coeffs: v, ring: self.clone() }
    }
}
