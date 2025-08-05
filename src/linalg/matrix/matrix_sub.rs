use std::ops::Sub;

use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, vector::util::non_zeroed_vec}};

impl<T: Number> Matrix<T> {
    pub fn gen_sub(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        assert!(a.well_formed());
        assert!(b.well_formed());
        assert_eq!(a.shape(), b.shape());
        let size = a.size();
        unsafe {
            Matrix::u_gen_sub(a, b, size)
        }
    }

    pub unsafe fn u_gen_sub(a: &Matrix<T>, b: &Matrix<T>, size: usize) -> Matrix<T> {
        unsafe {
            let mut des = Matrix {
                rows: a.rows,
                cols: a.cols,
                data: non_zeroed_vec(size),
            };
            Matrix::u_gen_isub(a, b, &mut des, size);
            des
        }
    }

    pub fn gen_isub(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>) {
        assert!(a.well_formed());
        assert!(b.well_formed());
        assert!(des.well_formed());
        assert_eq!(a.shape(), b.shape());
        assert_eq!(a.shape(), des.shape());
        unsafe {
            Matrix::u_gen_isub(a, b, des, a.size());
        }
    }

    pub unsafe fn u_gen_isub(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>, size: usize) {
        for i in 0..size {
            unsafe {
                *des.get_mut(i) = *a.get(i) - *b.get(i)
            }
        }
    }
}

impl<T: Number> Sub for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Matrix::gen_sub(self, rhs)
    }
}