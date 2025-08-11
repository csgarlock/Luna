use crate::{core::number::Number, linalg::vector::util::non_zeroed_vec};

#[inline(always)]
pub fn gen_vector_neg<T: Number>(a: &[T]) -> Vec<T> {
    unsafe { u_gen_vector_neg(a.as_ptr(), a.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_neg<T: Number>(a: *const T, size: usize) -> Vec<T> {
    unsafe {
        let mut v = non_zeroed_vec(size);
        u_gen_vector_ineg(a, v.as_mut_ptr(), size);
        v
    }
}

#[inline(always)]
pub fn gen_vector_ineg<T: Number>(a: &[T], des: &mut [T]) {
    assert_eq!(a.len(), des.len());
    unsafe { u_gen_vector_ineg(a.as_ptr(), des.as_mut_ptr(), a.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_ineg<T: Number>(a: *const T, des: *mut T, size: usize) {
    unsafe {
        for i in 0..size {
            *des.add(i) = -*a.add(i);
        }
    }
}