use std::fmt;
use std::ops::{Mul, Add, SubAssign, Div};
use std::cmp::PartialEq;
use bigdecimal::BigDecimal;
use num_traits::identities::{One, Zero};
use num_bigint::ToBigInt;


// Naive and basic implementation of Matrixs and Vectors

// Roadmap: 
// - Vecteur -> div/mul par un scalaire, normalisation
// - Création d'une matrice depuis un Vec<Vec<>>, depuis un Vec<Vector>
// - Mul Matrice/vecteur
// - Grahm-schmidt (Voir si il est possible de le faire avec seulement des entiers ? Ou bien on se fait un type rationnel ?)

#[derive(Clone, Debug)]
pub struct Matrix {
    pub data: Vec<Vec<BigDecimal>>,
    m: usize, // Rows
    n: usize, // Columns
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    pub data: Vec<BigDecimal>,
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self{
        let data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        Vector { data }
    }
}

impl SubAssign for Vector {

    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            data: self.data.iter().zip(other.data.iter()).map(|(x,y)| x - y).collect()
        };
    }
    
}

impl Div<BigDecimal> for Vector {
    type Output = Self;

    fn div(self, rhs: BigDecimal) -> Self {
        Vector { data: self.data.iter().map(|x| x / rhs.clone()).collect() }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {

        if self.n != rhs.m {
            panic!(format!("Can't multiply a {}x{} matrix with a {}x{} one.", self.m, self.n, rhs.m, rhs.n));
        }

        self.naive_mul(rhs)

    }
}

impl Mul<Matrix> for Vector {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Vector {
        if rhs.n != self.data.len() {
            panic!(format!("Can't multiply a {}x{} matrix with a {} dimension vector.", rhs.m, rhs.n, self.data.len()));
        }

        let m = Matrix::from_vector(&self);
        Vector::from_matrix(&(rhs * m))
    }
}


impl Mul<BigDecimal> for Vector {

    type Output = Self;

    fn mul(self, rhs: BigDecimal) -> Vector {
        Vector { data: self.data.iter().map(|x| x * rhs.clone()).collect() }
    }
}

impl Matrix {

    // Create a zero mxn matrix
    pub fn new(n: usize, m: usize) -> Self {
        Matrix { data: vec![vec![BigDecimal::zero(); n]; m],  m, n }
    }

    // Create a square Matrix with the given diagonal
    pub fn diag(d: &[BigDecimal]) -> Self {
        
        let n = d.len();
        let mut data = Vec::new();

        for i in 0..n {
            let mut row = vec![BigDecimal::zero(); n];
            row[i] = d[i].clone();
            data.push(row);
        }

        Matrix { data, n, m:n}
    }

    // Create the n x n identity Matrix
    pub fn identity(n: usize) -> Self {
        Matrix::diag(&vec![BigDecimal::one(); n])
    }

    // Create a new Matrix from a Vector
    pub fn from_vector(v: &Vector) -> Self {
        Matrix { data: vec![v.data.clone()], m:v.data.len(), n: 1 }
    }

    // Create a new Matrix with the given vectors as rows
    pub fn from_vectors_rows(v: &[&[BigDecimal]]) -> Self {
        // Todo:  verify that each vector have the same length

        Matrix {
            data: v.iter().map(|&x| Vec::from(x)).collect(),
            m: v.len(),
            n: v[0].len(),
        }
    }

    // Create a new Matrix with the given vectors as rows
    pub fn from_vectors_columns(_v: &[&[BigDecimal]]) -> Self {
        // TODO

        Matrix {
            data: Vec::new(),
            m: 0,
            n: 0,
        }
    }

    pub fn is_square(&self) -> bool {
        self.n == self.m
    }

    // Compute c = self * b, with the naive O(n^3) algorithm
    fn naive_mul(&self, b: Self) -> Self {
        
        let mut c = Matrix::new(self.m, b.n);

        for i in 0..(self.m - 1) {
            for k in 0..(b.n - 1) {

                let mut r = BigDecimal::zero();
                for j in 0..self.n {
                    r += self.data[i][j].clone() * b.data[j][k].clone();
                }

                c.data[i][k] = r;
            }
        }
        c
    }

    pub fn transpose(&self) -> Self {
        let mut r = Matrix::new(self.n, self.m);

        for i in 0..self.m {
            for j in 0..self.n {
                r.data[j][i] = self.data[i][j].clone();
            }
        }

       r
    }

    // Quel strat ? En place ? Renvoie ? Une de chaque? Une pour les deux avec un param?
    // pub fn row_echelon(&self) -> Se

    pub fn determinant(&self) -> Result<BigDecimal, &str> {

        if !self.is_square() {
            Err("Can't compute the determinant of a non-square matrix.")
        } else {
            Ok(BigDecimal::zero()) // TODO
        }
    }

    pub fn row_echelon(&self) -> Self {
        Matrix::new(0,0) // TODO
    }

}

impl Vector {

    pub fn new(data: &[BigDecimal]) -> Self {
        Vector { data: data.to_vec() }
    }

    pub fn from_matrix(m: &Matrix) -> Self {
        if m.n != 1 {
            panic!(format!("Can't cast a {}x{} matrix to a vector.", m.m, m.n));
        }

        Vector { data: m.data[0].clone() }
    }

    pub fn euclidean_norm(&self) -> BigDecimal {
        self.data.iter().map(|x| x*x).sum::<BigDecimal>().sqrt().unwrap()
    }

    pub fn normalize(&self) -> Self {
        self.clone() / self.euclidean_norm()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "Mat<{}x{}>: [", self.m, self.n)?;
        for row in self.data.clone() {
            writeln!(f, "{:?}", row)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        writeln!(f, "Vector<{}>: [", self.data.len())?;
        for val in self.data.clone() {
            writeln!(f, "{:?}", val)?;
        }
        write!(f, "]")
    }
}

pub fn round_nearest(n: &BigDecimal) -> BigDecimal {

    if (n - n.to_bigint().unwrap()).abs() > 0.5.into() {
        BigDecimal::from(n.to_bigint().unwrap() + 1)
    } else {
        BigDecimal::from(n.to_bigint().unwrap())
    }

}
