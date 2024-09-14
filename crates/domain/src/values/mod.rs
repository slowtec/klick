use derive_more::From;

pub mod constants;
pub mod specs;
pub mod units;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum ValueId {
    Custom(String),
    In(specs::InputValueId),
    Out(specs::OutputValueId),
}

impl ValueId {
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}
