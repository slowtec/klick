use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use derive_more::Display;
use leptos::*;

mod rendering;
pub use self::rendering::*;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[must_use]
fn unique_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[must_use]
pub fn dom_node_id() -> FieldId // String
{
    // DOM element IDs needs to be locally unique
    // within the HTML document.
    let id = unique_id();
    FieldId(id)
}

#[derive(Debug, Clone)]
pub struct FieldSet {
    pub title: Option<&'static str>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

impl Field {
    #[must_use]
    pub const fn unit(&self) -> Option<&'static str> {
        match self.field_type {
            FieldType::Float { unit, .. } | FieldType::UnsignedInteger { unit, .. } => Some(unit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display)]
#[display("field-{_0}")]
pub struct FieldId(usize);

#[derive(Debug, Clone, Copy)]
pub struct MinMax<T> {
    pub min: Option<T>,
    pub max: Option<T>,
}

// TODO: Rename to something like FieldRenderingData
#[derive(Debug, Clone)]
pub enum FieldType {
    Float {
        initial_value: Option<f64>,
        placeholder: Option<String>,
        limits: MinMax<f64>,
        unit: &'static str, // TODO: use presenter::ValueUnit trait
        on_change: Callback<Option<f64>, ()>,
        input: Signal<Option<f64>>,
    },
    UnsignedInteger {
        initial_value: Option<u64>,
        placeholder: Option<String>,
        limits: MinMax<u64>,
        unit: &'static str, // TODO: use presenter::ValueUnit trait
        on_change: Callback<Option<u64>, ()>,
        input: Signal<Option<u64>>,
    },
    Text {
        initial_value: Option<String>,
        placeholder: Option<String>,
        max_len: Option<usize>,
        on_change: Callback<Option<String>, ()>,
        input: Signal<Option<String>>,
    },
    Bool {
        initial_value: Option<bool>,
        on_change: Callback<bool, ()>,
        input: Signal<bool>,
    },
}
