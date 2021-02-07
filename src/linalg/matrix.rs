use std::ops::{Index,IndexMut,Add,Sub,Neg,Mul};

type Index2D = (usize, usize);

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    arr: Vec<f64>,
}

impl Matrix {
    /// Creates a flattened row-major order Matrix from a given 2D array
    pub fn from(array: &[&[f64]]) -> Self {
        let rows: usize = array.len();
        assert_ne!(rows, 0);
        let cols: usize = array[0].len();
        assert_ne!(cols, 0);
        let mut arr: Vec<f64> = vec![];
        for row in array {
            arr.extend_from_slice(row);
        }

        Self { rows, cols, arr }
    }

    /// Returns a rows-by-cols zero/null matrix
    pub fn zeros(rows: usize, cols: usize) -> Self {
        assert_ne!(rows, 0);
        assert_ne!(cols, 0);
        Self { rows, cols, arr: vec![0.0; rows * cols] }
    }

    /// Returns a rows-by-cols matrix of ones
    pub fn ones(rows: usize, cols: usize) -> Self {
        assert_ne!(rows, 0);
        assert_ne!(cols, 0);
        Self { rows, cols, arr: vec![1.0; rows * cols] }
    }

    /// Returns an n-by-n identity matrix
    pub fn eye(n: usize) -> Self {
        assert_ne!(n, 0);
        let mut result: Matrix = Matrix::zeros(n, n);
        for i in 0..result.rows {
            result[(i,i)] = 1.0;
        }
        result
    }

    /// Returns the transpose of the matrix
    pub fn transpose(&self) -> Self {
        let mut result: Matrix = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(j, i)] = self[(i, j)];
            }
        }
        result
    }

    /// Returns the Hadamard Product (Elementwise-multiplication) of 2 Matrices with the same shape
    pub fn hadamard(&self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let arr = self.arr.iter()
            .zip(other.arr.iter())
            .map(|(&x, &y)| x * y)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, arr }
    }

    /// Returns the trace of a square matrix, denoted tr(A)
    /// tr(A) is defined as the sum of the elements in the main diagonal
    pub fn trace(&self) -> f64 {
        assert_eq!(self.rows, self.cols);
        let mut result: f64 = 0.0;
        for i in 0..self.rows {
            result += self[(i, i)];
        }
        result
    }

    /// gaussian elimination
    pub fn gaussian_elimination(&mut self) {
        for i in 1..self.rows {
            for j in 0..i {
                if self[(i, j)] != 0.0 {
                    let r: f64 = self[(i, j)] / self[(j, j)];
                    for k in 0..self.cols {
                        self[(i, k)] = self[(i, k)] / r - self[(j, k)];
                    }
                }
            }
        }
    }

    /// Returns the determinant of a matrix m (Laplace expansion)
    pub fn determinant(m: &Self) -> f64 {
        assert_eq!(m.rows, m.cols);
        if m.rows == 1 {
            return m[(0,0)];
        } else {
            let mut result: f64 = 0.0;
            for i in 0..m.cols {
                let mut minor: Matrix = Matrix::zeros(m.cols-1, m.cols-1);
                for j in 1..m.cols {
                    for k in 0..m.cols {
                        if k == i {
                            continue;
                        }
                        minor[(j-1, if k < i {k} else {k-1})] = m[(j,k)];
                    }
                }
                result += (if i as f64 % 2.0 == 0.0 {1.0} else {-1.0}) * m[(0, i)] * Matrix::determinant(&minor);
            }
            result
        }
    }
}

/// Allows one to index into the array in "immutable contexts"
/// Usage: matrix[(i, j)]
impl Index<Index2D> for Matrix {
    type Output = f64;
    fn index(&self, (i, j): Index2D) -> &Self::Output {
        &self.arr[i * self.cols + j]
    }
}

/// Allows one to index into the array in "mutable contexts"
/// Usage: matrix[(i, j)] = x
impl IndexMut<Index2D> for Matrix {
    fn index_mut(&mut self, (i, j): Index2D) -> &mut Self::Output {
        &mut self.arr[i * self.cols + j]
    }
}

/// Returns the elementwise negation of all entries in the Matrix
impl Neg for &Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        let arr = self.arr.iter()
            .map(|&x| -x)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, arr }
    }
}

/// Adds two matrices elementwise
impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let arr = self.arr.iter()
            .zip(other.arr.iter())
            .map(|(&x, &y)| x + y)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, arr }
    }
}

/// Subtracts one matrix from the other elementwise
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, other: Self) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let arr = self.arr.iter()
            .zip(other.arr.iter())
            .map(|(&x, &y)| x - y)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, arr }
    }
}

/// Returns the result of a scalar * Matrix
impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f64) -> Matrix {
        let arr = self.arr.iter()
            .map(|&x| scalar * x)
            .collect();
        Matrix { rows: self.rows, cols: self.cols, arr }
    }
}

/// Returns the result of Matrix * scalar
impl Mul<&Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, m: &Matrix) -> Matrix {
        let arr = m.arr.iter()
            .map(|&x| self * x)
            .collect();
        Matrix { rows: m.rows, cols: m.cols, arr }
    }
}

/// Returns the product of two matrices
impl Mul for &Matrix { 
    type Output = Matrix;
    fn mul(self, other: Self) -> Matrix {
        assert_eq!(self.cols, other.rows);
        let mut result: Matrix = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[(i, j)] += self[(i, k)] * other[(k, j)];
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        // Indexing
        let mut a: Matrix = Matrix::from(&[&[1.0,2.0,3.0], &[4.0,5.0,6.0]]);
        assert_eq!(a[(0,0)], 1.0);
        a[(0,0)] = 42.0;
        assert_eq!(a[(0,0)], 42.0);
        
        // Addition
        let b: Matrix = Matrix::ones(a.rows, a.cols);
        assert_eq!(&a + &b, Matrix::from(&[&[43.0,3.0,4.0], &[5.0,6.0,7.0]]));

        // Hadamard Product
        let c: Matrix = Matrix::from(&[&[1.0,4.0,7.0], &[8.0,20.0,5.0]]);
        let d: Matrix = a.hadamard(c);
        assert_eq!(d, Matrix::from(&[&[42.0,8.0,21.0], &[32.0,100.0,30.0]]));

        // Negation
        let e: Matrix = -&d;
        assert_eq!(e, Matrix::from(&[&[-42.0,-8.0,-21.0], &[-32.0,-100.0,-30.0]]));

        // Scalar Multiplication
        assert_eq!(1.5 * &e, Matrix::from(&[&[-63.0,-12.0,-31.5], &[-48.0,-150.0,-45.0]]));

        // Matrix, Matrix Multiplication
        let f: Matrix = &Matrix::from(&[&[1.0,2.0],&[3.0,4.0]]) * &Matrix::from(&[&[4.0,5.0],&[6.0,7.0]]);
        assert_eq!(f, Matrix::from(&[&[16.0,19.0],&[36.0,43.0]]));
        let g: Matrix = Matrix::from(&[&[4.0,5.0,6.0,7.0],&[8.0,9.0,10.0,11.0],&[12.0,13.0,14.0,15.0]]);
        assert_eq!(&a * &g, Matrix::from(&[&[220.0,267.0,314.0,361.0],&[128.0,143.0,158.0,173.0]]));

        // Transpose
        let a_t: Matrix = a.transpose();
        assert_eq!(a_t.transpose(), a);

        // Identity Matrix
        assert_eq!(Matrix::eye(3), Matrix::from(&[&[1.0,0.0,0.0], &[0.0,1.0,0.0], &[0.0,0.0,1.0]]));

        // Trace
        assert_eq!(f.trace(), 59.0);

        // Determinant
        assert_eq!(Matrix::determinant(&f), 4.0);
        let h: Matrix = Matrix::from(&[&[2.0, 4.0, 3.0, 3.0], &[2.0, 4.0, 4.0, 0.0], &[0.0, 9.0, 7.0, 8.0], &[0.0, 5.0, 3.0, 7.0]]);
        assert_eq!(Matrix::determinant(&h), 2.0);

        // Gaussian Elimination
        let mut i: Matrix = Matrix::from(&[&[2.0,-1.0,1.0],&[1.0,1.0,5.0]]);
        &i.gaussian_elimination();
        dbg!(i);
    }
}