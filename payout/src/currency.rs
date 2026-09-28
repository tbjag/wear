use std::{fmt, ops::{Add, Div, Neg, Rem, Sub, SubAssign}};

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Eq, Ord)]
pub struct Currency {
    cents: i64
}

impl Currency {
    pub fn new(value: f32) -> Self {
        let cents = (value * 100.0).round() as i64;
        Currency{cents}
    }

    pub fn zero() -> Self {
        Currency{cents: 0.into()}
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Currency{ cents: self.cents + rhs.cents}
    }
}

impl Div for Currency {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Currency{ cents: self.cents / rhs.cents }
    }
}

impl Rem for Currency {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        Currency{ cents: self.cents % rhs.cents }
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Currency{ cents: self.cents - rhs.cents }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cents as f32 / 100.0)
    }
}

impl Neg for Currency {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Currency{ cents: -self.cents}
    }
}

impl SubAssign for Currency {
    fn sub_assign(&mut self, rhs: Self) {
        self.cents -= rhs.cents
    }
}

impl Div<i64> for Currency {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output {
        Currency{cents: self.cents / rhs}
    }
}

impl Rem<i64> for Currency {
    type Output = Self;
    fn rem(self, rhs: i64) -> Self::Output {
        Currency{cents: self.cents % rhs}
    }
}