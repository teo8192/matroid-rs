use num_integer::{gcd, Integer};

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Rational<I: Integer + Clone> {
    numerator: I,
    denominator: I,
}

impl<I: Integer + Clone> Rational<I> {
    fn simplify(self) -> Self {
        if self.numerator == self.denominator {
            Rational {
                numerator: I::one(),
                denominator: I::one(),
            }
        } else if self.numerator == I::zero() {
            Rational {
                numerator: I::zero(),
                denominator: I::one(),
            }
        } else {
            let gcd = gcd(self.numerator.clone(), self.denominator.clone());
            Rational {
                numerator: self.numerator / gcd.clone(),
                denominator: self.denominator / gcd,
            }
        }
    }

    fn inverse(self) -> Self {
        assert!(self.numerator != I::zero());
        Rational {
            numerator: self.denominator,
            denominator: self.numerator,
        }
    }

    #[allow(unused)]
    pub fn is_integer(&self) -> bool {
        let clone = self.clone().simplify();
        clone.denominator == I::one() || clone.denominator == I::zero() - I::one()
    }

    pub fn to_integer(&self) -> Option<I> {
        let number = self.clone().simplify();
        if number.denominator == I::one() {
            Some(number.numerator)
        } else if number.denominator == I::zero() - I::one() {
            Some(I::zero() - number.numerator)
        } else {
            None
        }
    }

    pub fn exp(self, n: i32) -> Self {
        match n {
            0 => Rational {
                numerator: I::one(),
                denominator: I::one(),
            },
            1 => self,
            n if n < 0 => self.inverse().exp(-n),
            n => {
                let mut result = self.clone();
                for _ in 1..n {
                    result = result * self.clone();
                }
                result
            }
        }
    }

    pub fn from(numerator: I) -> Self {
        Rational {
            numerator,
            denominator: I::one(),
        }
    }
}

impl<I: Integer + Clone + Display> Display for Rational<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(i) = self.clone().simplify().to_integer() {
            write!(f, "{}", i)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl<I: Integer + Clone> PartialEq for Rational<I> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            == self.denominator.clone() * other.numerator.clone()
    }
}

impl<I: Integer + Clone> Eq for Rational<I> {}

impl<I: Integer + Clone> Add for Rational<I> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let numerator = self.numerator.clone() * other.denominator.clone()
            + self.denominator.clone() * other.numerator.clone();
        let denominator = self.denominator * other.denominator;
        Rational {
            numerator,
            denominator,
        }
        .simplify()
    }
}

impl<I: Integer + Clone> Neg for Rational<I> {
    type Output = Self;

    fn neg(self) -> Self {
        Rational {
            numerator: I::zero() - self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<I: Integer + Clone> Sub for Rational<I> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl<I: Integer + Clone> Mul for Rational<I> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let numerator = self.numerator.clone() * other.numerator.clone();
        let denominator = self.denominator * other.denominator;
        Rational {
            numerator,
            denominator,
        }
        .simplify()
    }
}

impl<I: Integer + Clone> Div for Rational<I> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: Self) -> Self {
        self * other.inverse()
    }
}

impl<I: Integer + From<u8> + Clone> From<u8> for Rational<I> {
    fn from(n: u8) -> Self {
        Rational {
            numerator: n.into(),
            denominator: I::one(),
        }
    }
}
