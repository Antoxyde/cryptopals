use num_bigint::BigInt;
use std::default::Default;
use std::ops::{Add, Mul, Sub};
use num_traits::Zero;
use num_integer::binomial;
use num_traits::pow::Pow;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnivariatePoly {
    pub coeffs: Vec<BigInt>,
}

impl Default for UnivariatePoly {

    fn default() -> Self { 
        UnivariatePoly { coeffs: vec![Zero::zero()] }
    }

}

impl Add for UnivariatePoly {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        
        let mut r = Vec::new();

        for d in 0..std::cmp::max(self.degree(), rhs.degree()) as usize {
            r.push(self.coeff_deg(d) + rhs.coeff_deg(d));
        }

        UnivariatePoly { coeffs: r}.remove_leading_zeroes()
    }
}

impl Sub for UnivariatePoly {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut r = Vec::new();

        for d in 0..std::cmp::max(self.degree(), rhs.degree()) as usize {
            r.push(self.coeff_deg(d) - rhs.coeff_deg(d));
        }

        UnivariatePoly { coeffs: r}.remove_leading_zeroes()
    }
}

impl Add<BigInt> for UnivariatePoly {
    type Output = Self;

    fn add(self, rhs: BigInt) -> Self {
        let mut v = self.coeffs;
        if v.is_empty() {
            v.push(rhs);
        } else {
            v[0] += rhs;
        }
        
        UnivariatePoly { coeffs: v }
    }
}

impl Sub<BigInt> for UnivariatePoly {
    type Output = Self;

    fn sub(self, rhs: BigInt) -> Self {
        let mut v = self.coeffs;
        v[0] -= rhs;
        UnivariatePoly { coeffs: v }
    }
}


impl Mul<BigInt> for UnivariatePoly {

    type Output = Self;

    fn mul(self, rhs: BigInt) -> Self {
        UnivariatePoly { coeffs: self.coeffs.iter().map(|x| x * rhs.clone()).collect() }
    }
}



impl Zero for UnivariatePoly {

    fn zero() -> Self {
        Self::default()
    }

    fn is_zero(&self) -> bool {
        self.coeffs.iter().filter(|&x| !x.is_zero()).count() == 0
    }

}

impl fmt::Display for UnivariatePoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut res  = String::new();
        res.push_str(&format!( "{} + ", self.coeffs[0]));

       for (degree, coeff) in self.coeffs[1..].iter().enumerate() {
            res.push_str(&format!("{}x^{} + ", coeff, degree + 1));
       }

       write!(f,"{}", &res)
    }
}

impl UnivariatePoly {

    pub fn new(coeffs: &[BigInt]) -> Self {
        UnivariatePoly {
            coeffs: coeffs.to_vec()
        }
    }

    pub fn gcd(&self, other: &Self) -> Self {
        let mut r0 = self.clone();
        let mut r1 = other.clone();

        loop {
            let (_,r) = r0.clone().euclid_div(r1.clone()); 
            if r == Zero::zero() {
                return r0;
            }

            r0 = r1;
            r1 = r;
        }
    }

    fn euclid_div(self, rhs: Self) -> (Self, Self) {

        println!("Euclid div : {} / {}", self, rhs);
        
        if rhs.is_zero() {
            panic!("Can't divide by zero polynomial.");
        }

        let d = rhs;

        let mut q = UnivariatePoly::default();
        let mut r = self;

        while !r.is_zero() && r.degree() >= d.degree() {
            let t = r.lead() / d.lead();
            q = q.clone() + t.clone();

            r = r - d.clone() * t;
        }

        (q,r)
    }


    // Return the coefficient of degree deg
    pub fn coeff_deg(&self, deg: usize) -> BigInt {
        if self.coeffs.len() > deg {
            self.coeffs[deg].clone()
        } else {
            Zero::zero()
        }
    }

    fn remove_leading_zeroes(&self) -> Self {
        
        for (index, coeff) in self.coeffs.iter().rev().enumerate() {
            if *coeff != Zero::zero() {
                return UnivariatePoly {coeffs: self.coeffs[..(self.coeffs.len() - index)].to_vec() }
            }
            
        }

        UnivariatePoly::default()
        
    }

    // If poly.is_zero() return -1
    pub fn degree(&self) -> i64 {
        self.remove_leading_zeroes();
        (self.coeffs.len() as i64) - 1
    }

    pub fn lead(&self) -> BigInt {
        let v = self.remove_leading_zeroes();
        
        v.coeffs.last().unwrap().clone()
    }

}

// Expands (x + b)^n 
pub fn expands_binomial(b: &BigInt, n: u64) -> UnivariatePoly {

    let mut coeffs = Vec::new();
    for k in 0..=n {
        coeffs.push(binomial(n, k) * b.pow(n - k));
    }  

    UnivariatePoly { coeffs }
}
