use std::{fmt, ops::{Index, IndexMut}};

use crate::core::number::Number;

pub struct Matrix<T: Number> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>
}

impl<T: Number> Matrix<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len());
        Self { rows: rows, cols: cols, data: data }
    }

    pub unsafe fn get(&self, index: usize) -> &T {
        unsafe {
            self.data.get_unchecked(index)
        }
    }

    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        unsafe {
            self.data.get_unchecked_mut(index)
        }
    }

    pub unsafe fn get_loc(&self, index: (usize, usize)) -> &T {
        unsafe {
            self.data.get_unchecked(index.0 * self.cols + index.1)
        }
    }

    pub unsafe fn get_mut_loc(&mut self, index: (usize, usize)) -> &mut T {
        unsafe {
            self.data.get_unchecked_mut(index.0 * self.cols + index.1)
        }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn well_formed(&self) -> bool {
        self.size() == self.data.len()
    }

    pub fn index_to_location(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
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

impl<T: Number> Index<usize> for Matrix<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T:Number> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Number + fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "[")?;
            for j in 0..self.cols {
                write!(f, "{}", self[(i, j)])?;
                if j < self.cols - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if i < self.rows - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}


