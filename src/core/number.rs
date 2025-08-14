use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

pub trait Number:
    Copy
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + PartialEq<Self>
    + PartialOrd<Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn inverse(self) -> Self;
}

impl Number for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn inverse(self) -> Self {
        1.0 / self
    }
}

impl Number for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn inverse(self) -> Self {
        1.0 / self
    }
}

pub trait RealNumber: Number {
    fn power(self, other: Self) -> Self;
}    


impl RealNumber for f32 {
    fn power(self, other: Self) -> Self { f32::powf(self, other) }
}

impl RealNumber for f64 {
    fn power(self, other: Self) -> Self { f64::powf(self, other) }
}