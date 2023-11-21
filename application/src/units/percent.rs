use std::{fmt, ops::Mul};

use crate::Factor;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percent(f64);

impl Percent {
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_factor(&self) -> Factor {
        Factor::new(self.0 / 100.0)
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

impl Mul<Percent> for &f64 {
    type Output = f64;
    fn mul(self, value: Percent) -> Self::Output {
        self * value.as_factor()
    }
}

impl Mul<&Percent> for &f64 {
    type Output = f64;
    fn mul(self, value: &Percent) -> Self::Output {
        self * value.as_factor()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        let value = Percent::new(55.2);
        assert_eq!(format!("{value}"), "55.2%");
    }
}
