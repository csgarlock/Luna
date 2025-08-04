use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::core::number::{Number, RealNumber};

#[derive(Clone, Copy, Debug)]
pub struct Complex<T: RealNumber> {
    pub re: T,
    pub im: T,
}

impl<T: RealNumber> Complex<T> {
    pub fn conjugate(self) -> Complex<T> {
        Complex { re: self.re, im: -self.im }
    }
}

impl<T: RealNumber> Add<> for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{ re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

impl<T: RealNumber> Sub<> for Complex<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { re: self.re - rhs.re, im: self.im - rhs.im }
    }
}

impl<T: RealNumber> Mul<> for Complex<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im + rhs.re,
        } 
    }
}

impl<T: RealNumber> Div<> for Complex<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let dem = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / dem,
            im: (self.im * rhs.re - self.re * rhs.im) / dem,
        }
    }
}

impl<T: RealNumber> Neg<> for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { re: -self.re, im: -self.im }
    }
}

pub trait ComplexNumber: Number {
    fn conjugate(self) -> Self;
}

impl<T: RealNumber> ComplexNumber for Complex<T> {
    fn conjugate(self) -> Self {
        self.conjugate()
    }
}
