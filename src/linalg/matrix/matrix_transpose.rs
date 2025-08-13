use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, vector::util::non_zeroed_vec}};

impl<T: Number> Matrix<T> {

    #[inline(always)]
    pub fn transpose(&mut self) {
        assert!(self.well_formed());
        unsafe {
            self.u_transpose(self.size());
        }
    }

    #[inline(always)]
    pub unsafe fn u_transpose(&mut self, _size: usize) {
        todo!();
    }
}

#[inline(always)]
pub fn gen_matrix_transpose<T: Number>(a: &Matrix<T>) -> Matrix<T> {
    assert!(a.well_formed());
    let size = a.size();
    unsafe {
        u_gen_matrix_transpose(a, size)
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_transpose<T: Number>(a: &Matrix<T>, size: usize) -> Matrix<T> {
    unsafe {
        let mut des = Matrix {
            rows: a.cols,
            cols: a.rows,
            data: non_zeroed_vec(size),
        };
        u_gen_matrix_itranspose(a, &mut des, size);
        des
    }
}

#[inline(always)]
pub fn gen_matrix_itranspose<T: Number>(a: &Matrix<T>, des: &mut Matrix<T>) {
    assert!(a.well_formed());
    assert!(des.well_formed());
    assert_eq!(a.rows, des.cols);
    assert_eq!(a.cols, des.rows);
    unsafe {
        u_gen_matrix_itranspose(a, des, a.size());
    }
}

#[inline(always)]
pub unsafe fn u_gen_matrix_itranspose<T: Number>(a: &Matrix<T>, des: &mut Matrix<T>, size: usize) {
    for i in 0..size {
        unsafe {
            let loc = a.index_to_location(i);
            *des.get_mut(i) = *a.get_loc(loc.1, loc.0);
        }
    }
}