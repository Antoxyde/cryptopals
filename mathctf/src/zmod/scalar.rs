use crate::zmod::ring::ZModRing;
use crate::arithmetic::inv_mod;

use num_bigint::BigUint;
use std::ops::{Add, Mul, Sub, Div, Neg, AddAssign};
use num_traits::{Zero, Pow};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZModScalar {
    pub ring: ZModRing,
    pub value: BigUint,
}

impl Pow<ZModScalar> for ZModScalar {
    type Output = ZModScalar;
    fn pow(self, rhs: ZModScalar) -> ZModScalar {
        self.ring.scalar(&self.value.modpow(&rhs.value,&self.ring.modulus))
    }
}

impl Pow<u64> for ZModScalar {
    type Output = ZModScalar;
    fn pow(self, rhs: u64) -> ZModScalar {
        self.ring.scalar(&self.value.modpow(&BigUint::from(rhs),&self.ring.modulus))
    }
}

impl Zero for ZModScalar {

    fn zero() -> Self {
        panic!("Not implemented: use ring.zero() instead");
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl Neg for ZModScalar {
    type Output = ZModScalar;

    fn neg(self) -> Self {
        self.ring.scalar(&(&self.ring.modulus - self.value))
    }
}

impl Add for ZModScalar {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.ring.scalar(&(self.value + rhs.value))
    }
}

impl AddAssign for ZModScalar {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for ZModScalar {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {

        let tmp = if rhs.value > self.value {
            self.value.clone() + &self.ring.modulus
        } else {
            self.value.clone()
        };

        self.ring.scalar(&(tmp - rhs.value))
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for ZModScalar {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.ring.scalar(&(self.value * rhs.value))
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for ZModScalar {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let inv = rhs.invert().unwrap_or_else(|| panic!("Can't divide by {} because it's not invertible in Z/{}Z", rhs, self.ring));
        self.ring.scalar(&(self.value * inv.value))

    }
}

impl fmt::Display for ZModScalar {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ZModScalar {

    pub fn invert(&self)  -> Option<ZModScalar> {
        if let Some(inverted) = inv_mod(&self.value, &self.ring.modulus) {
            Some(self.ring.scalar(&inverted))
        } else {
            None
        }
    }
    
}

