use crate::{core::number::Number, linalg::vector::util::non_zeroed_vec};

#[inline(always)]
pub fn gen_vector_add<T: Number>(a: &[T], b: &[T]) -> Vec<T> {
    assert_eq!(a.len(), b.len());
    unsafe { u_gen_vector_add(a.as_ptr(), b.as_ptr(), a.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_add<T: Number>(a: *const T, b: *const T, size: usize) -> Vec<T> {
    unsafe {
        let mut v = non_zeroed_vec(size);
        u_gen_vector_iadd(a, b, v.as_mut_ptr(), size);
        v
    }
}

#[inline(always)]
pub fn gen_vector_iadd<T: Number>(a: &[T], b: &[T], des: &mut [T]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), des.len());
    unsafe { u_gen_vector_iadd(a.as_ptr(), b.as_ptr(), des.as_mut_ptr(), a.len()) }
}

#[inline(always)]
pub unsafe fn u_gen_vector_iadd<T: Number>(a: *const T, b: *const T, des: *mut T, size: usize) {
    unsafe {
        for i in 0..size {
            *des.add(i) = *a.add(i) + *b.add(i);
        }
    }
}