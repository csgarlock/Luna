use std::{fmt::Debug, ops::{Add, Div, Mul, Neg, Sub}};

use crate::core::complex::Complex;

pub trait Number:
    Copy
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Number for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Number for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl<T: RealNumber> Number for Complex<T> {
    fn zero() -> Self { Self { re: T::zero(), im: T::zero() } }
    fn one() -> Self { Self { re: T::one(), im: T::one() } }
}

pub trait RealNumber: Number {
    fn power(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
}    


impl RealNumber for f32 {
    fn power(self, other: Self) -> Self { f32::powf(self, other) }
    fn sin(self) -> Self { f32::sin(self) }
    fn cos(self) -> Self { f32::cos(self) }
    fn tan(self) -> Self { f32::tan(self) }
    fn asin(self) -> Self { f32::asin(self) }
    fn acos(self) -> Self { f32::acos(self) }
    fn atan(self) -> Self { f32::atan(self) }
}

impl RealNumber for f64 {
    fn power(self, other: Self) -> Self { f64::powf(self, other) }
    fn sin(self) -> Self { f64::sin(self) }
    fn cos(self) -> Self { f64::cos(self) }
    fn tan(self) -> Self { f64::tan(self) }
    fn asin(self) -> Self { f64::asin(self) }
    fn acos(self) -> Self { f64::acos(self) }
    fn atan(self) -> Self { f64::atan(self) }
}