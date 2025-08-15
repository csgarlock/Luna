use crate::{core::number::Number, linalg::{matrix::matrix::Matrix, solvers::linear_system_solution::LinearSystemSolution, vector::util::non_zeroed_vec}};

pub fn solve_linear_system_rref<T: Number>(aug_matrix: &Matrix<T>) -> Option<Vec<T>> {
    assert!(aug_matrix.well_formed());
    unsafe { u_solve_linear_system_rref(aug_matrix, aug_matrix.rows, aug_matrix.cols) }
}

pub unsafe fn u_solve_linear_system_rref<T: Number>(aug_matrix: &Matrix<T>, rows: usize, cols: usize) -> Option<Vec<T>> {
    let matrix_copy = &mut aug_matrix.clone();
    let mut des_vector = unsafe { non_zeroed_vec::<T>(cols - 1) };
    if unsafe { u_isolve_linear_system_rref(matrix_copy, des_vector.as_mut_ptr(), rows, cols) } {
        Some(des_vector)
    } else {
        None
    }
}

pub fn isolve_linear_system_rref<T: Number>(aug_matrix: &mut Matrix<T>, answer_des: &mut Vec<T>) -> bool {
    assert!(aug_matrix.well_formed());
    assert_eq!(answer_des.len(), aug_matrix.cols - 1);
    unsafe { u_isolve_linear_system_rref(aug_matrix, answer_des.as_mut_ptr(), aug_matrix.rows, aug_matrix.cols) }
}

pub unsafe fn u_isolve_linear_system_rref<T: Number>(aug_matrix: &mut Matrix<T>, answer_des: *mut T, rows: usize, cols: usize) -> bool {
    unsafe {
        aug_matrix.u_reduced_row_echelon_form(rows, cols);
    }
    let last_col = cols - 1;
    let mut current_col = 0;
    let mut row_offset;
    unsafe {
        for row in 0..rows {
            row_offset = row * cols;
            while *aug_matrix.get(row_offset + current_col) != T::one() {
                *answer_des.add(current_col) = T::zero();
                current_col += 1;
                if current_col == last_col {
                    if *aug_matrix.get(row_offset + current_col) != T::zero() {
                        return false;
                    }
                    return true;
                }
            }
            *answer_des.add(current_col) = *aug_matrix.get(row_offset + last_col);
            current_col += 1;
        }
    }
    true

}

pub fn solve_linear_system_basis_rref<T: Number>(aug_matrix: &Matrix<T>) -> LinearSystemSolution<T> {
    assert!(aug_matrix.well_formed());
    unsafe { u_solve_linear_system_basis_rref(aug_matrix, aug_matrix.rows, aug_matrix.cols) }
}

pub unsafe fn u_solve_linear_system_basis_rref<T: Number>(aug_matrix: &Matrix<T>, rows: usize, cols: usize) -> LinearSystemSolution<T> {
    let matrix_copy = &mut aug_matrix.clone();
    unsafe { u_isolve_linear_system_basis_rref(matrix_copy, rows, cols) }
}

pub fn isolve_linear_system_basis_rref<T: Number>(aug_matrix: &mut Matrix<T>) -> LinearSystemSolution<T> {
    assert!(aug_matrix.well_formed());
    unsafe { u_isolve_linear_system_basis_rref(aug_matrix, aug_matrix.rows, aug_matrix.cols) }
}

pub unsafe fn u_isolve_linear_system_basis_rref<T: Number>(aug_matrix: &mut Matrix<T>, rows: usize, cols: usize) -> LinearSystemSolution<T> {
    unsafe {
        aug_matrix.u_reduced_row_echelon_form(rows, cols);
    }
    let last_col = cols - 1;
    let mut current_col = 0;
    let mut row_offset;
    let mut answer_vec: Vec<T> = unsafe { non_zeroed_vec(last_col) };
    let mut basis_vecs = Vec::new();
    let mut infinite_solutions = false;
    let mut free_lookup = vec![false; last_col];
    let mut free_list = Vec::with_capacity(last_col);
    let mut homogeneous = true;
    unsafe {
        'outer: for row in 0..rows {
            row_offset = row * cols;
            while *aug_matrix.get(row_offset + current_col) != T::one() {
                infinite_solutions = true;
                *free_lookup.get_unchecked_mut(current_col) = true;
                let mut basis_vec = vec![T::zero(); last_col];
                *basis_vec.get_unchecked_mut(current_col) = T::one();
                basis_vecs.push(basis_vec);
                free_list.push(current_col);
                current_col += 1;
                if current_col == last_col {
                    if *aug_matrix.get(row_offset + current_col) != T::zero() {
                        return LinearSystemSolution::None;
                    }
                    break 'outer;
                }
            }
            let temp = *aug_matrix.get(row_offset + last_col);
            *answer_vec.get_unchecked_mut(current_col) = temp;
            if temp != T::zero() {
                homogeneous = false;
            }
            current_col += 1;
        }
        for col in current_col..last_col {
            infinite_solutions = true;
            *free_lookup.get_unchecked_mut(col) = true;
            let mut basis_vec = vec![T::zero(); last_col];
            *basis_vec.get_unchecked_mut(col) = T::one();
            basis_vecs.push(basis_vec);
            free_list.push(col);
        }
        if !infinite_solutions {
            LinearSystemSolution::Solution(answer_vec)
        } else {
            for (basis, col) in basis_vecs.iter_mut().zip(free_list.iter()) {
                let mut row = 0;
                for i in 0..last_col {
                    if !free_lookup.get_unchecked(i) {
                        *basis.get_unchecked_mut(i) = -*aug_matrix.get_loc(row, *col);
                        row += 1;
                    }
                }
            }
            if homogeneous {
                LinearSystemSolution::HomogenousBasis(basis_vecs)
            } else {
                LinearSystemSolution::AffineBasis(answer_vec, basis_vecs)
            }
        }

    }
}