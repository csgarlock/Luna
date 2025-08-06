use std::ops::Sub;

use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, vector::util::non_zeroed_vec}};

impl<T: Number> Matrix<T> {
    #[inline(always)]
    pub fn sub(&mut self, other: &Matrix<T>) {
        assert!(self.well_formed());
        assert!(other.well_formed());
        assert_eq!(self.shape(), other.shape());
        unsafe {
            self.u_sub(other, self.size());
        }
    }

    #[inline(always)]
    pub unsafe fn u_sub(&mut self, other: &Matrix<T>, size: usize) {
        for i in 0..size {
            unsafe {
                *self.get_mut(i) = *self.get(i) - *other.get(i);
            }
        }
    }
}

#[inline(always)]
pub fn gen_matrix_sub<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
    assert!(a.well_formed());
    assert!(b.well_formed());
    assert_eq!(a.shape(), b.shape());
    let size = a.size();
    unsafe {
        u_gen_matrix_sub(a, b, size)
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_sub<T: Number>(a: &Matrix<T>, b: &Matrix<T>, size: usize) -> Matrix<T> {
    unsafe {
        let mut des = Matrix {
            rows: a.rows,
            cols: a.cols,
            data: non_zeroed_vec(size),
        };
        u_gen_matrix_isub(a, b, &mut des, size);
        des
    }
}

#[inline(always)]
pub fn gen_matrix_isub<T: Number>(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>) {
    assert!(a.well_formed());
    assert!(b.well_formed());
    assert!(des.well_formed());
    assert_eq!(a.shape(), b.shape());
    assert_eq!(a.shape(), des.shape());
    unsafe {
        u_gen_matrix_isub(a, b, des, a.size());
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_isub<T: Number>(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>, size: usize) {
    for i in 0..size {
        unsafe {
            *des.get_mut(i) = *a.get(i) - *b.get(i)
        }
    }
}

impl<T: Number> Sub for &Matrix<T> {
    type Output = Matrix<T>;
    
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        gen_matrix_sub(self, rhs)
    }
}