use num_bigint::{ToBigUint, BigUint, BigInt, Sign};
use num_traits::{One, Zero};


// Classical GCD with i32
pub fn gcd(mut n1: i32, mut n2: i32) -> i32 {

   while n1 != 0 {
       let old_n1 = n1;
       n1 = n2 % n1;
       n2 = old_n1;
   }

   n2.abs()
}

#[allow(clippy::many_single_char_names)]
pub fn big_egcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {

    let mut s: BigInt = Zero::zero();
    let mut old_s: BigInt = One::one();
    let mut r: BigInt = b.clone();
    let mut old_r: BigInt = a.clone();

    while r != Zero::zero() {

        let quotient = old_r.clone() / r.clone();

        let tmp_r = r.clone();
        r = old_r - quotient.clone() * tmp_r.clone();
        old_r = tmp_r;

        let tmp_s = s;
        s = old_s - quotient * tmp_s.clone();
        old_s = tmp_s;

    }

    let u = old_s.clone();
    let gcd = old_r.clone();

    let v = if b.clone() == Zero::zero() {
        Zero::zero()
    } else {
        (old_r - old_s * a) / b
    };

    (gcd, u, v)
}

// Inverse modulo, compute x such that x * a = 1 (mod m)
#[allow(clippy::many_single_char_names)]
pub fn inv_mod(a: &BigUint, m: &BigUint) -> Option<BigUint> {

    let (g, mut x, _) = big_egcd(&BigInt::from_biguint(Sign::Plus, a.clone() % m), &BigInt::from_biguint(Sign::Plus,m.clone()));

    if g != One::one() {
        None
    } else {
        if x < Zero::zero() {
            x += BigInt::from_biguint(Sign::Plus, m.clone());
        }

        Some(x.to_biguint().expect("wut") % m)
    }
}

pub fn chinese_remainder(modulis: &[BigUint], residues: &[BigUint]) -> Option<BigUint> {

    let prod: BigUint = modulis.iter().product();

    let mut sum: BigUint = Zero::zero();

    for (residue, moduli) in residues.iter().zip(modulis) {
        let p = &prod / moduli;
        sum += residue * inv_mod(&p, moduli).expect("Modulis aren't pairwise coprimes") * p;
    }

    Some(sum % prod)
}



pub fn naive_prime_factorisation(m: &BigUint) -> Vec<BigUint> {
    let mut primes = Vec::new();

    let mut n = m.clone();
    let mut i = 2.to_biguint().unwrap();

    loop {
        if &n % i.clone() == BigUint::zero() {
            primes.push(i.clone());
            n /= i;
            i = BigUint::one();
        }

        if i >= n {
            break;
        }

        i += BigUint::one();
    }
    
    primes
}

