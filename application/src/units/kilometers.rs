/// Kilometers `[km]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kilometers(f64);

impl Kilometers {
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Kilometers> for f64 {
    fn from(from: Kilometers) -> Self {
        from.0
    }
}
