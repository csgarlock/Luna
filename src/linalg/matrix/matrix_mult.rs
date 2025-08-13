use std::ops::Mul;

use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, vector::{util::non_zeroed_vec, vector_dot::u_gen_vector_sdot}}};

#[inline(always)]
pub fn gen_matrix_mult<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
    assert!(a.well_formed());
    assert!(b.well_formed());
    assert_eq!(a.cols, b.rows);
    unsafe {
        u_gen_matrix_mult(a, b, a.rows, a.cols, b.cols)
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_mult<T: Number>(a: &Matrix<T>, b: &Matrix<T>, m: usize, n: usize, p:usize) -> Matrix<T> {
    unsafe {
        let mut des = Matrix {
            rows: a.rows,
            cols: b.cols,
            data: non_zeroed_vec(m * p),
        };
        u_gen_matrix_imult(a, b, &mut des, m, n, p);
        des
    }
}

#[inline(always)]
pub fn gen_matrix_imult<T: Number>(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>) {
    assert!(a.well_formed());
    assert!(b.well_formed());
    assert!(des.well_formed());
    assert_eq!(a.cols, b.rows);
    assert_eq!(a.rows, des.rows);
    assert_eq!(b.cols, des.cols);
    unsafe {
        u_gen_matrix_imult(a, b, des, a.rows, a.cols, b.cols);
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_imult<T: Number>(a: &Matrix<T>, b: &Matrix<T>, des: &mut Matrix<T>, m: usize, n: usize, p: usize) {
    let mut a_ptr = unsafe { a.data.as_ptr().sub(n) };
    let b_ptr = b.data.as_ptr();
    for i in 0..m*p {
        unsafe {
            if i % p == 0 {
                a_ptr = a_ptr.add(n)
            }
            *des.get_mut(i) = u_gen_vector_sdot(a_ptr, b_ptr.add(i % p), 1, p, n)
        }
    }
}


impl<T: Number> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        gen_matrix_mult(self, rhs)
    }
}