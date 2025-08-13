use crate::{core::number::Number, linalg::matrix::matrix::Matrix};

impl<T: Number> Matrix<T> {

    pub fn row_echelon_form(&mut self) {
        assert!(self.well_formed());
        unsafe { self.u_row_echelon_form(self.rows, self.cols) };
    }

    pub unsafe fn u_row_echelon_form(&mut self, rows: usize, cols: usize) {
        let mut start_col: usize = 0;
        let mut start_row: usize = 0;
        let mut row_offset: usize = 0;
        unsafe {
            while start_row < rows - 1 && start_col < cols {
                let mut max_index = start_row;
                let mut max_val = *self.get(row_offset);
                for row in start_row+1..rows {
                    let test_val = *self.get(start_col + row * cols);
                    if test_val > max_val {
                        max_index = row;
                        max_val = test_val;
                    }
                }
                if max_val != T::zero() {
                    self.u_swap_rows(max_index, start_row, cols);
                    for row in start_row+1..rows {
                        let inner_row_offset = row * cols;
                        let mult_value = *self.get(inner_row_offset + start_col) * max_val.inverse();
                        *self.get_mut(inner_row_offset + start_col) = T::zero();
                        for col in start_col+1..cols {
                            let sub_val = mult_value * *self.get(row_offset + col);
                            *self.get_mut(inner_row_offset + col) -= sub_val;
                        }
                    }
                    start_row += 1;
                    row_offset += cols;
                }
                start_col += 1;
            }
        }
    }

    pub fn reduced_row_echelon_form(&mut self) {
        assert!(self.well_formed());
        unsafe { self.u_reduced_row_echelon_form(self.rows, self.cols) };
    }

    pub unsafe fn u_reduced_row_echelon_form(&mut self, rows: usize, cols: usize) {
        let mut start_col: usize = 0;
        let mut start_row: usize = 0;
        let mut row_offset: usize = 0;
        unsafe {
            while start_row < rows && start_col < cols {
                let mut max_index = start_row;
                let mut max_val = *self.get(row_offset + start_col);
                for row in start_row+1..rows {
                    let test_val = *self.get(start_col + row * cols);
                    if test_val > max_val {
                        max_index = row;
                        max_val = test_val;
                    }
                }
                if max_val != T::zero() {
                    self.u_swap_rows(max_index, start_row, cols);
                    for row in 0..rows {
                        if row != start_row {
                            let inner_row_offset = row * cols;
                            let mult_value = *self.get(inner_row_offset + start_col) * max_val.inverse();
                            *self.get_mut(inner_row_offset + start_col) = T::zero();
                            for col in start_col+1..cols {
                                let sub_val = mult_value * *self.get(row_offset + col);
                                *self.get_mut(inner_row_offset + col) -= sub_val;
                            }
                        }
                    }
                    *self.get_mut(row_offset + start_col) = T::one();
                    let mult_value = max_val.inverse();
                    for col in start_col+1..cols {
                        *self.get_mut(row_offset + col) *= mult_value;
                    }
                    start_row += 1;
                    row_offset += cols;
                }
                start_col += 1;
            }
        }
    }
}