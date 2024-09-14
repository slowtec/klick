use derive_more::From;

mod project;
mod specs;

pub mod authentication;
pub mod constants;
pub mod units;

pub use self::{
    project::{Id as ProjectId, IdParseError as ProjectIdParseError, Project},
    specs::{InputValueId, OutputValueId},
    units::{Value, ValueType},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Id {
    Custom(String),
    In(InputValueId),
    Out(OutputValueId),
}

impl Id {
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }
}
