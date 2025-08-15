use crate::core::number::Number;

pub enum LinearSystemSolution<T: Number> {
    AffineBasis(Vec<T>, Vec<Vec<T>>),
    HomogenousBasis(Vec<Vec<T>>),
    Solution(Vec<T>),
    None,
}