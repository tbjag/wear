use std::{fmt, ops::{Add, Div, Rem, Sub}};

#[derive(Debug, PartialEq)]
pub struct Currency(i64);

impl Currency {
    pub fn new(value: f32) -> Self {
        let cents = (value * 100.0).round() as i64;
        Currency(cents)
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0)
    }
}

impl Div for Currency {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Currency(self.0 / rhs.0)
    }
}

impl Rem for Currency {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Currency(self.0 % rhs.0)
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Currency(self.0 - rhs.0)
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as f32 / 100.0)
    }
}