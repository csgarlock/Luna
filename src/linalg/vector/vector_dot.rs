use crate::core::number::Number;

#[inline(always)]
pub fn gen_vector_dot<T: Number>(a: &[T], b: &[T]) -> T {
    assert_eq!(a.len(), b.len());
    unsafe {
        u_gen_vector_dot(a.as_ptr(), b.as_ptr(), a.len())
    }
}

#[inline(always)]
pub unsafe fn u_gen_vector_dot<T: Number>(a: *const T, b: *const T, size: usize) -> T {
    let mut acc= T::zero();
    unsafe {
        for i in 0..size {
            acc = acc + (*a.add(i) * *b.add(i));
        }
    }
    acc
}

#[inline(always)]
pub fn gen_vector_sdot<T: Number>(a: &[T], b: &[T], a_stride: usize, b_stride: usize) -> T {
    assert_eq!(a.len() / a_stride, b.len() / b_stride);
    unsafe {
        u_gen_vector_sdot(a.as_ptr(), b.as_ptr(), a_stride, b_stride, a.len() / a_stride)
    }
}

#[inline(always)]
pub unsafe fn u_gen_vector_sdot<T: Number>(a: *const T, b: *const T, a_stride: usize, b_stride: usize, size: usize) -> T {
    let mut acc= T::zero();
    unsafe {
        for i in 0..size {
            acc = acc + (*a.add(i*a_stride) * *b.add(i*b_stride));
        }
    }
    acc
}