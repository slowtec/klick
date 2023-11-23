/// Tons `[t]`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tons(f64);

impl Tons {
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }
}

impl From<Tons> for f64 {
    fn from(from: Tons) -> Self {
        from.0
    }
}
