use crate::{core::number::Number, linalg::matrix::matrix::Matrix};

impl<T: Number> Matrix<T> {
    pub fn zeroes(rows: usize, cols: usize) -> Self {
        Self { 
            rows: rows,
            cols: cols,
            data: vec![T::zero(); rows * cols] 
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut m = Self::zeroes(size, size);
        for i in 0..size {
            m[(i, i)] = T::one();
        }
        m
    }
}