use std::ops::Mul;

/// A dimensionless factor.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Factor(f64);

impl Factor {
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Factor> for f64 {
    fn from(from: Factor) -> Self {
        from.0
    }
}

impl Mul<f64> for Factor {
    type Output = f64;

    fn mul(self, rhs: f64) -> f64 {
        self.0 * rhs
    }
}

impl Mul<Factor> for f64 {
    type Output = f64;

    fn mul(self, rhs: Factor) -> f64 {
        self * rhs.0
    }
}

impl Mul<Factor> for &f64 {
    type Output = f64;

    fn mul(self, rhs: Factor) -> f64 {
        self * rhs.0
    }
}
