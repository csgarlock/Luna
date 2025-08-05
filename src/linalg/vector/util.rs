use crate::core::number::Number;

pub unsafe fn non_zeroed_vec<T: Number>(size: usize) -> Vec<T> {
    let mut v = Vec::with_capacity(size);
    unsafe {
        v.set_len(size);
    }
    v
}