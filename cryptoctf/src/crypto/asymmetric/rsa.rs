use num_bigint::BigUint;
use mathctf::arithmetic::inv_mod;

pub struct RsaPublicKey {
    n: BigUint,
    e: BigUint,
}

pub struct RsaPrivateKey {
    n: BigUint,
    e: BigUint,
    p: BigUint,
    q: BigUint,
    d: BigUint,
}

impl RsaPublicKey {

    pub fn new(p: &BigUint, q: &BigUint, e: &BigUint) -> RsaPublicKey {
        RsaPublicKey {n : p*q, e: e.clone()}
    }

    pub fn encrypt(&self, data : &[u8]) -> Vec<u8> {
        let m  = BigUint::from_bytes_be(data);
        m.modpow(&self.e, &self.n).to_bytes_be()
    }

}

impl RsaPrivateKey {
    #[allow(clippy::many_single_char_names)]
    pub fn new(p: &BigUint, q: &BigUint, e:&BigUint) -> RsaPrivateKey {

        let n = p * q;
        // Todo : verify that e is prime pairwise to n
        let d = inv_mod(e, &n).expect("e and n aren't coprimes.");

        RsaPrivateKey {
            n,
            e:e.clone(),
            d,
            p:p.clone(),
            q:q.clone(),
        }
    }
}
