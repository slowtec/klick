use derive_more::From;

mod project;

pub mod authentication;

pub use self::project::{Id as ProjectId, IdParseError as ProjectIdParseError, Project};

pub use klick_value::{
    constants,
    specs::{InputValueId, OutputValueId},
    units::{self, Value, ValueType},
};

pub mod input_value {
    pub use klick_value::{
        extract_optional_with_input_value_id as optional,
        extract_required_with_input_value_id as required, specs::InputValueId,
    };
}

pub mod output_value {
    pub use klick_value::{
        extract_optional_with_output_value_id as optional,
        extract_required_with_output_value_id as required, specs::OutputValueId,
    };
}

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
