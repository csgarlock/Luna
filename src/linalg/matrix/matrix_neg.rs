use std::ops::Neg;

use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, vector::util::non_zeroed_vec}};

impl<T: Number> Matrix<T> {

    #[inline(always)]
    pub fn negate(&mut self) {
        assert!(self.well_formed());
        unsafe {
            self.u_negate(self.size());
        }
    }

    #[inline(always)]
    pub unsafe fn u_negate(&mut self, size: usize) {
        for i in 0..size {
            unsafe {
                *self.get_mut(i) = -*self.get(i);
            }
        }
    }
}

#[inline(always)]
pub fn gen_matrix_neg<T: Number>(a: &Matrix<T>) -> Matrix<T> {
    assert!(a.well_formed());
    let size = a.size();
    unsafe {
        u_gen_matrix_neg(a, size)
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_neg<T: Number>(a: &Matrix<T>, size: usize) -> Matrix<T> {
    unsafe {
        let mut des = Matrix {
            rows: a.rows,
            cols: a.cols,
            data: non_zeroed_vec(size),
        };
        u_gen_matrix_ineg(a, &mut des, size);
        des
    }
}

#[inline(always)]
pub fn gen_matrix_ineg<T: Number>(a: &Matrix<T>, des: &mut Matrix<T>) {
    assert!(a.well_formed());
    assert!(des.well_formed());
    assert_eq!(a.shape(), des.shape());
    unsafe {
        u_gen_matrix_ineg(a, des, a.size());
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_ineg<T: Number>(a: &Matrix<T>, des: &mut Matrix<T>, size: usize) {
    for i in 0..size {
        unsafe {
            *des.get_mut(i) = -*a.get(i);
        }
    }
}

impl<T: Number> Neg for &Matrix<T> {
    type Output = Matrix<T>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        gen_matrix_neg(self)
    }
}