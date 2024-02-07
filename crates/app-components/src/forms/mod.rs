use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use inflector::cases::kebabcase::to_kebab_case;
use leptos::*;

mod rendering;
pub use self::rendering::*;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn dom_node_id<ID>(field_id: &ID) -> String
where
    ID: Copy + AsRef<str>,
{
    // DOM element IDs needs to be locally unique
    // within the HTML document.
    let id = unique_id();

    // The name is only for humans for better readability.
    let name = to_kebab_case(field_id.as_ref());

    format!("{name}-{id}")
}

#[derive(Debug, Clone)]
pub struct FieldSet<ID> {
    pub title: Option<&'static str>,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Clone)]
pub struct Field<ID> {
    pub id: ID,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

// TODO: use presenter::ValueUnit trait
impl<ID> Field<ID> {
    pub const fn unit(&self) -> Option<&'static str> {
        match self.field_type {
            FieldType::Float { unit, .. } => Some(unit),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct RequiredField<ID>
where
    ID: AsRef<str> + Copy + Hash + Eq,
{
    pub id: ID,
    pub field_id: String, //FIXME: rename dom_node_id
}

#[derive(Clone)]
pub struct MissingField {
    pub field_id: String, //FIXME: rename dom_node_id
    pub label: String,    // TODO: use presenter::ValueLabel
}

impl MissingField {
    pub const fn new(field_id: String, label: String) -> Self {
        Self { field_id, label }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MinMax<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum FieldType {
    Float {
        initial_value: Option<f64>,
        placeholder: Option<&'static str>,
        limits: MinMax<f64>,
        unit: &'static str, // TODO: use presenter::ValueUnit trait
    },
    UnsignedInteger {
        initial_value: Option<u64>,
        placeholder: Option<&'static str>,
        limits: MinMax<u64>,
        unit: &'static str, // TODO: use presenter::ValueUnit trait
    },
    Text {
        initial_value: Option<String>,
        placeholder: Option<&'static str>,
        max_len: Option<usize>,
    },
    Bool {
        initial_value: Option<bool>,
    },
    // TODO: support enums
    Selection {
        initial_value: Option<usize>,
        options: Vec<SelectOption>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct SelectOption {
    pub label: &'static str,
    pub value: usize,
}

#[derive(Clone, Copy)]
pub enum FieldSignal {
    Float {
        input: RwSignal<Option<String>>,
        output: RwSignal<Option<f64>>,
    },
    UnsignedInteger {
        input: RwSignal<Option<String>>,
        output: RwSignal<Option<u64>>,
    },
    Text(RwSignal<Option<String>>),
    Bool(RwSignal<bool>),
    Selection(RwSignal<Option<usize>>),
}

impl FieldSignal {
    pub fn get_float(&self) -> Option<f64> {
        match self {
            Self::Float { output, .. } => output.get(),
            _ => None,
        }
    }

    pub fn get_unsigned_integer(&self) -> Option<u64> {
        match self {
            Self::UnsignedInteger { output, .. } => output.get(),
            _ => None,
        }
    }

    pub fn get_text(&self) -> Option<String> {
        match self {
            Self::Text(s) => s.get(),
            _ => None,
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(s) => Some(s.get()),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn get_selection(&self) -> Option<usize> {
        match self {
            Self::Selection(s) => s.get(),
            _ => None,
        }
    }

    // TODO: rename to : get_float_input_signal
    pub const fn get_float_signal(&self) -> Option<RwSignal<Option<String>>> {
        match self {
            Self::Float { input, .. } => Some(*input),
            _ => None,
        }
    }

    pub const fn get_float_output_signal(&self) -> Option<RwSignal<Option<f64>>> {
        match self {
            Self::Float { output, .. } => Some(*output),
            _ => None,
        }
    }

    // TODO: rename to get_unsigned_integer_input_signal
    pub const fn get_unsigned_integer_signal(&self) -> Option<RwSignal<Option<String>>> {
        match self {
            Self::UnsignedInteger { input, .. } => Some(*input),
            _ => None,
        }
    }

    pub const fn get_text_signal(&self) -> Option<RwSignal<Option<String>>> {
        match self {
            Self::Text(s) => Some(*s),
            _ => None,
        }
    }

    pub const fn get_bool_signal(&self) -> Option<RwSignal<bool>> {
        match self {
            Self::Bool(s) => Some(*s),
            _ => None,
        }
    }

    pub const fn get_selection_signal(&self) -> Option<RwSignal<Option<usize>>> {
        match self {
            Self::Selection(s) => Some(*s),
            _ => None,
        }
    }

    pub fn clear(&self) {
        match self {
            Self::Float { input, .. } => input.set(None),
            Self::UnsignedInteger { input, .. } => input.set(None),
            Self::Text(s) => s.set(None),
            Self::Bool(s) => s.set(false),
            Self::Selection(s) => s.set(None),
        }
    }

    pub fn as_formatted_string(&self) -> Option<String> {
        match self {
            Self::Float { input, .. } => input.get(),
            Self::UnsignedInteger { input, .. } => input.get(),
            Self::Text(s) => s.get(),
            Self::Bool(s) => Some(if s.get() { "Ja" } else { "Nein" }.to_string()),
            Self::Selection(_) => todo!(),
        }
    }
}
