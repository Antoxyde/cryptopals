use crate::zmod::scalar::ZModScalar;
use crate::zmod::ring::ZModRing;

use num_traits::{Zero, One, Pow};
use std::ops::{Add, Mul, Sub, Div, AddAssign};
use std::fmt;
use num_bigint::BigUint;
use std::convert::{Into, From};


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZModPoly {
    pub coeffs: Vec<ZModScalar>,
    pub ring: ZModRing,
}


impl Pow<usize> for ZModPoly {
    type Output = ZModPoly;
    fn pow(self, rhs: usize) -> ZModPoly {

        if self != self.ring.x() {
            panic!("Implemented only for monic poly of degree 1 (X) atm.");
        }

        if rhs == 0 {
            self.ring.one().into()
        } else if rhs == 1 {
            self
        } else {
            ZModPoly { coeffs: [vec![self.ring.zero(); rhs], vec![self.ring.one()]].concat(), ring: self.ring }
        }

    }
}

impl Zero for ZModPoly {

    fn zero() -> Self {
        // We can't implement this because that imply a default ModRing to put the default PolyModRingElement in
        // And the only solution i've found is to set the modulus of this default ModRing to 0 for infinity
        // but this just add too much complexity to the code
        // And i'm still implementing the zero traits because  is_zero() is really helpfuls
        panic!("Not implemented: use ring.zero_poly() instead");
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().filter(|&x| !x.is_zero()).count() == 0
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for ZModPoly {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {

        let mut r = Vec::new();

        for d in 0..(std::cmp::max(self.degree(), rhs.degree()) + 1) as usize {
            r.push(self.coeff_deg(d) + rhs.coeff_deg(d));
        }
        ZModPoly { coeffs: r, ring: self.ring}.remove_leading_zeroes()
    }
}

impl Sub<ZModScalar> for ZModPoly {
    type Output = Self;

    fn sub(self, rhs: ZModScalar) -> Self {

        let mut v = self.coeffs;
        v[0] = v[0].clone() - rhs;
        ZModPoly { coeffs: v, ring: self.ring }
    }
}


#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for ZModPoly {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut p = Vec::new();
        for deg in 0..=std::cmp::max(rhs.degree(), self.degree()) {
            p.push(self.coeff_deg(deg as usize) - rhs.coeff_deg(deg as usize));
        }
        ZModPoly { coeffs: p, ring: self.ring }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for ZModPoly {
    type Output = Self;

    // Naive poly mul
    fn mul(self, rhs: Self) -> Self {

        let mut p: Vec<ZModScalar> = Vec::new();

        for deg_a in 0..=self.degree()  as usize {
            for deg_b in 0..=rhs.degree() as usize {

                if (deg_a + deg_b) as usize >= p.len() {
                    p.push(self.ring.zero());
                }

                p[deg_a + deg_b] =  p[deg_a + deg_b].clone() + self.coeff_deg(deg_a) * rhs.coeff_deg(deg_b);
            }
        }
        ZModPoly { coeffs: p, ring: self.ring }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul<ZModScalar> for ZModPoly {
    type Output = Self;

    // Naive poly mul
    fn mul(self, rhs: ZModScalar) -> Self {

        let mut coeffs = Vec::new();

        for deg in 0..=self.degree()  as usize {
            coeffs.push(self.coeff_deg(deg) * rhs.clone());
        }

        ZModPoly { coeffs, ring: self.ring }
    }
}


impl Div<ZModScalar> for ZModPoly {
    type Output = Self;

    fn div(self, rhs: ZModScalar) -> Self {

        if rhs.is_zero() {
            panic!("Cant divide by zero.");
        }

        let mut v = Vec::new();
        for coeff in self.coeffs.iter() {
            v.push(coeff.clone() / rhs.clone());
        }

        ZModPoly { coeffs: v, ring: self.ring }

    }
}

impl AddAssign for ZModPoly {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}


impl fmt::Display for ZModPoly {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut res = String::new();

        if self.degree() >= 0 {
           res.push_str(&format!("{} + ", self.coeffs[0]));
            for (degree, coeff) in self.coeffs[1..].iter().enumerate() {
                if !coeff.is_zero() {
                    if *coeff == self.ring.scalar(&BigUint::one()) {
                        res.push_str(&format!("x^{} + ", degree + 1));
                    }  else {
                        res.push_str(&format!("{}x^{} + ", coeff, degree + 1));
                    }
                }
            }
        } else {
            res.push_str("0");

        }

        write!(f, "{}", &res)
    }
}


impl ZModPoly {

    pub fn gcd(&self, other: &Self) -> Self {
        //println!("gcd call with a = {}, b = {}", self, other);

        if other.is_zero() {
            self.monic()
        } else {

            let (_quo,rem) = self.euclid_div(other);
            //println!("a % b = {}", rem);
            //println!("a / b = {}", quo);
            other.gcd(&rem)
        }
    }
    #[allow(clippy::many_single_char_names)]
    fn euclid_div(&self, rhs: &Self) -> (Self, Self) {
        //println!("euclid_div call {} / {}", self, rhs);
        if rhs.is_zero() {
            panic!("Can't divide by zero polynomial.");
        }

        let mut q = ZModPoly::from(self.ring.zero());
        let mut r = self.clone();
        let d = rhs.degree();
        let c = rhs.lead();

        while r.degree() >= d {
            
            let mut s = self.ring.x().pow( (r.degree() - d) as usize);
            s = s.clone() * (r.lead() / c.clone());
            q += s.clone();
            r = r - (rhs.clone() * s);
            //println!("euclid_div r = {}, q = {}", r, q);
        }

        (q,r)
    }

    // Return the coefficient of degree deg
    pub fn coeff_deg(&self, deg: usize) -> ZModScalar {
        if self.coeffs.len() > deg {
            self.coeffs[deg].clone()
        } else {
            self.ring.zero()
        }
    }

    pub fn remove_leading_zeroes(&self) -> Self {
        
        for (index, coeff) in self.coeffs.iter().rev().enumerate() {
            if !coeff.is_zero() {
                let r = ZModPoly {coeffs: self.coeffs[..(self.coeffs.len() - index)].to_vec(), ring: self.ring.clone() };
                return r;
            }
        }

        self.ring.zero().into()
    }

    // if poly.is_zero() return -1
    pub fn degree(&self) -> i64 {
        let v = self.remove_leading_zeroes();
        if v.is_zero() {
            -1
        } else {
            (v.coeffs.len() as i64) - 1
        }
        
    }

    pub fn lead(&self) -> ZModScalar {
        let v = self.remove_leading_zeroes();
        v.coeffs.last().unwrap().clone()
    }

    pub fn monic(&self) -> ZModPoly {
        let v = self.remove_leading_zeroes();
        v.clone() / v.lead()
    }

}

impl From<ZModScalar> for ZModPoly {
    fn from(s: ZModScalar) -> ZModPoly {
        ZModPoly {ring: s.ring.clone(), coeffs: vec![s],  }
    }
}

impl Into<ZModScalar> for ZModPoly {
    
    fn into(self) -> ZModScalar {

        if self.degree() != 0 {
            panic!("Can't convert a ZModPoly of degree {} to ZModScalar.", self.degree())
        } 

        self.coeffs[0].clone()
    }
}
