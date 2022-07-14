use crate::primitives::*;
use bigdecimal::BigDecimal;

// Canonical scalar product of R^n, aka inner product or dot product in the general case
pub fn scalar_product(u: &Vector, v: &Vector) -> BigDecimal {
    u.data.iter().zip(v.data.iter()).map(|(x,y)| x*y).sum()
}

pub fn orthogonal_projection(u: &Vector, v: &Vector) -> Vector {
    u.clone() * (scalar_product(v, u) / scalar_product(u,u))
}

// Naive implementation of the classical Gram-Schmidt process
pub fn classical_gram_schmidt(b: &[Vector], normalize: bool) -> Vec<Vector> {

    let mut b_ortho = Vec::new();

    for (i, bb) in b.iter().enumerate() {

        let mut proj = bb.clone();

        for bb_ortho in b_ortho.iter().take(i) {
            proj -= orthogonal_projection(bb_ortho, &b[i]);
        }

        if normalize {
            b_ortho.push(proj.normalize());
        } else {
            b_ortho.push(proj);
        }

    }

    b_ortho
}

// LLL utility
fn size_reduce(basis: &mut [Vector]) {

    let n = basis.len();

    for j in 2..=n {
        for i in (1..j).rev() {

            // A less naive impl would not recompute the GS every loop but its gud enouf
            let b_tilde = classical_gram_schmidt(&basis, false);
            let mu_coeff = round_nearest(&(scalar_product(&basis[j - 1], &b_tilde[i - 1]) / scalar_product(&b_tilde[i - 1], &b_tilde[i - 1])));
            basis[j - 1] -= basis[i - 1].clone() * mu_coeff;
        }
    }
}

// Naive implementation of LLL
pub fn lll(input_basis: &[Vector], gamma: &BigDecimal) -> Vec<Vector> {

    let mut basis = Vec::from(input_basis);
    let n = basis.len();

    let mut cont = true;

    while cont {

        // Reduction step
        size_reduce(&mut basis);

        cont = false;
        let b_tilde = classical_gram_schmidt(&basis, false);

        for i in 0..(n - 1) {

            // Checking the Lovasz condition for every vector

            let bi_tilde = b_tilde[i].euclidean_norm();
            let mu_i1_i = scalar_product(&basis[i + 1], &b_tilde[i]) / scalar_product(&b_tilde[i], &b_tilde[i]);
            let rhs = (b_tilde[i].clone() * mu_i1_i  + b_tilde[i + 1].clone()).euclidean_norm();
            
            if bi_tilde.clone() * bi_tilde.clone() * gamma > rhs.clone() * rhs.clone() {

                // If the condition is not met, swap and come back to reduce step

                basis.swap(i, i+1);
                cont = true;
                break;
            }
        }

    }

    basis
}
