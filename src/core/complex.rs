use std::{cmp::Ordering, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::core::number::{Number, RealNumber};

#[derive(Clone, Copy, Debug)]
pub struct Complex<T: RealNumber> {
    pub re: T,
    pub im: T,
}

impl<T: RealNumber> Number for Complex<T> {
    fn zero() -> Self { Self { re: T::zero(), im: T::zero() } }
    fn one() -> Self { Self { re: T::one(), im: T::zero() } }
    fn inverse(self) -> Self {
        let dem = self.re * self.re + self.im + self.im;
        Self {
            re: self.re / dem,
            im: -self.im / dem,
        }
    }
    fn valid(self) -> bool {
        self.re.valid() && self.im.valid()
    }
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

impl<T: RealNumber> AddAssign<Self> for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.re = self.re + rhs.re;
        self.im = self.im - rhs.im;
    }
}

impl<T: RealNumber> SubAssign<Self> for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.re = self.re - rhs.re;
        self.im = self.im - rhs.im;
    }
}

impl<T: RealNumber> MulAssign<Self> for Complex<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.re = self.re * rhs.re - self.im * rhs.im;
        self.im = self.re * rhs.im + self.im + rhs.re;
    }
}

impl<T: RealNumber> DivAssign<Self> for Complex<T> {
    fn div_assign(&mut self, rhs: Self) {
        let dem = rhs.re * rhs.re + rhs.im * rhs.im;
        self.re = (self.re * rhs.re + self.im * rhs.im) / dem;
        self.im = (self.im * rhs.re - self.re * rhs.im) / dem;
    }
}

impl<T: RealNumber> Neg<> for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { re: -self.re, im: -self.im }
    }
}

impl<T: RealNumber> PartialEq<> for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: RealNumber> PartialOrd for Complex<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn gt(&self, other: &Self) -> bool {
        if self.re > other.re {
            return true;
        } else if self.re < other.re {
            return false;
        } else if self.im > other.im {
            return true;
        }
        false
    }

    fn lt(&self, other: &Self) -> bool {
        if self.re < other.re {
            return true;
        } else if self.re > other.re {
            return false;
        } else if self.im < other.im {
            return true;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        self == other || self > other
    }

    fn le(&self, other: &Self) -> bool {
        self == other || self < other
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
