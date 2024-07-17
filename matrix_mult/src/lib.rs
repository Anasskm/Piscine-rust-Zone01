use std::ops::Mul;

use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T: Scalar> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::zero(); other.number_of_cols()]; self.number_of_rows()];

        for i in 0..self.number_of_rows() {
            for j in 0..other.number_of_cols() {
                for k in 0..self.number_of_cols() {
                    result[i][j] = result[i][j] + self.0[i][k] * other.0[k][j];
                }
            }
        }

        Some(Matrix(result))
    }
}
