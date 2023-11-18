use serde::{Deserialize, Serialize};

mod input;
mod output;
mod scenario;

mod export;
mod import;

pub use self::{
    export::{export_to_string_pretty, export_to_vec_pretty},
    import::{import_from_slice, import_from_str, Error as ImportError},
    input::*,
    output::*,
    scenario::*,
};

#[cfg(feature = "conversion")]
mod conversion;

pub const CURRENT_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSet<ID> {
    pub title: &'static str,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MinMax {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    Float {
        initial_value: Option<f64>,
        placeholder: Option<&'static str>,
        limits: MinMax,
        unit: &'static str,
    },
    Text {
        initial_value: Option<String>,
        placeholder: Option<&'static str>,
        max_len: Option<usize>,
    },
    Bool {
        initial_value: Option<bool>,
    },
    Selection {
        initial_value: Option<usize>,
        options: Vec<SelectOption>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SelectOption {
    pub label: &'static str,
    pub value: usize,
}
