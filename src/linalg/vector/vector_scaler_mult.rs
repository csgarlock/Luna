use crate::{core::number::Number, linalg::vector::util::non_zeroed_vec};

#[inline(always)]
pub fn gen_vector_scaler_mult<T: Number>(a: T, v: &[T]) -> Vec<T> {
    unsafe { u_gen_vector_scaler_mult(a, v.as_ptr(), v.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_scaler_mult<T: Number>(a: T, v: *const T, size: usize) -> Vec<T> {
    unsafe {
        let mut des = non_zeroed_vec(size);
        u_gen_vector_scaler_imult(a, v, des.as_mut_ptr(), size);
        des
    }
}

#[inline(always)]
pub fn gen_vector_scaler_imult<T: Number>(a: T, v: &[T], des: &mut [T]) {
    assert_eq!(v.len(), des.len());
    unsafe { u_gen_vector_scaler_imult(a, v.as_ptr(), des.as_mut_ptr(), v.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_scaler_imult<T: Number>(a: T, v: *const T, des: *mut T, size: usize) {
    unsafe {
        for i in 0..size {
            *des.add(i) = a * *v.add(i);
        }
    }
}