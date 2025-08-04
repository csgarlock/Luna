use std::ops::{Index, IndexMut};

use crate::core::number::Number;

pub struct Matrix<T: Number> {
    rows: usize,
    cols: usize,
    data: Vec<T>
}

impl<T: Number> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows: rows, cols: cols, data: data }
    }

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

    pub fn shape(self) -> (usize, usize) {
        (self.rows, self.cols)
    }
    
}

impl<T: Number> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}

impl<T: Number> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 * self.cols + index.1]
    }
}