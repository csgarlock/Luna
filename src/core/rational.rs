use std::{cmp::Ordering, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use crate::core::number::Number;

#[derive(Clone, Copy, Debug)]
pub struct Rational {
    pub pos: bool,
    pub num: u64,
    pub den: u64,
}

impl Number for Rational {
    fn zero() -> Self { Self { pos: true, num: 0, den: 0 } }
    fn one() -> Self { Self {pos: true, num: 1, den: 1} }
    fn inverse(self) -> Self { Self { pos: self.pos, num: self.den, den: self.num } }
}

impl Rational {
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn lcm(a: u64, b: u64) -> u64 {
        (a / Rational::gcd(a, b)) * b
    }

    fn matching_num(&self, other: &Rational) -> (u64, u64, u64) {
        let lcm = Rational::lcm(self.den, other.den);
        (lcm, self.num * (lcm / self.den), other.num * (lcm / other.den))
    }

    pub fn simplify(&mut self) {
        if self.den != 0 {
            let gcd = Rational::gcd(self.num, self.den);
            self.num /= gcd;
            self.den /= gcd;
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let nums = Rational::matching_num(&self, &rhs);
        let mut output;
        match (self.pos, rhs.pos) {
          (true, true) => output = Self { pos: true, num: nums.1 + nums.2, den: nums.0 },
          (true, false) | (false, true) => {
            if nums.1 > nums.2 {
                output = Self { pos: true, num: nums.1 - nums.2, den: nums.0 }
            } else {
                output = Self { pos: false, num: nums.2 - nums.1, den: nums.0 }
            }
          },
          (false, false) => output = Self { pos: false, num: nums.1 + nums.2, den: nums.0 },
        }
        output.simplify();
        output
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Self {
            pos: match (self.pos, rhs.pos) {
                (true, true) | (false, false) => true,
                (true, false) | (false, true) => false,
            },
            num: self.num * rhs.num,
            den: self.den * rhs.den
        };
        output.simplify();
        output
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl AddAssign for Rational {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Rational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {pos: !self.pos, num: self.num, den: self.den }
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.pos != other.pos {
            false
        } else if self.num * other.den == other.num * self.den {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for Rational {
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
        match (self.pos, other.pos) {
            (true, true) => self.num * other.den > other.num * self.den,
            (false, false) => self.num * other.den < other.num * self.den,
            (true, false) => true,
            (false, true) => false,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (self.pos, other.pos) {
            (true, true) => self.num * other.den < other.num * self.den,
            (false, false) => self.num * other.den > other.num * self.den,
            (true, false) => true,
            (false, true) => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self.pos, other.pos) {
            (true, true) => self.num * other.den >= other.num * self.den,
            (false, false) => self.num * other.den <= other.num * self.den,
            (true, false) => true,
            (false, true) => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self.pos, other.pos) {
            (true, true) => self.num * other.den <= other.num * self.den,
            (false, false) => self.num * other.den >= other.num * self.den,
            (true, false) => true,
            (false, true) => false,
        }
    }
}