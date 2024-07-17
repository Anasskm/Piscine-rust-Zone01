use lalgebra_scalar::*;

// Assuming the Scalar trait is defined as follows:

#[derive(Debug, Clone,PartialEq,Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + std::clone::Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}
