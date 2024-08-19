mod calculation;
mod project;
mod values;

pub mod authentication;

pub use self::{
    calculation::*,
    project::{Id as ProjectId, IdParseError as ProjectIdParseError, Project},
    values::*,
};

pub use klick_value::{
    constants,
    specs::{InputValueId, MissingInputValueIdError, MissingOutputValueIdError, OutputValueId},
    units::{self, Value, ValueType},
};

pub mod input_value {
    pub use klick_value::{
        extract_optional_with_input_value_id as optional,
        extract_required_with_input_value_id as required,
        specs::{InputValueId, MissingInputValueIdError},
    };
}

pub mod output_value {
    pub use klick_value::{
        extract_optional_with_output_value_id as optional,
        extract_required_with_output_value_id as required,
        specs::{MissingOutputValueIdError, OutputValueId},
    };
}
