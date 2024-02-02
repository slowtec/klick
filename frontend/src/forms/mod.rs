use std::{
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use inflector::cases::kebabcase::to_kebab_case;
use leptos::wasm_bindgen::JsCast;
use leptos::*;

mod rendering;

pub use self::rendering::*;

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

pub fn form_field_id<ID>(field_id: &ID) -> String
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

#[derive(Clone)]
pub struct RequiredField<ID>
where
    ID: AsRef<str> + Copy + Hash + Eq,
{
    pub id: ID,
    pub field_id: String,
    pub label: &'static str,
}

#[derive(Clone)]
pub struct MissingField {
    pub field_id: String,
    pub label: &'static str,
}

impl MissingField {
    pub const fn new(field_id: String, label: &'static str) -> Self {
        Self { field_id, label }
    }
}

#[component]
pub fn HelperWidget<F>(missing_fields: Vec<MissingField>, before_focus: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    view! {
      <ul class="ml-5 my-4 list-disc list-inside">
        <For
          each = move || missing_fields.clone()
          key = |e| e.label.to_string()
          let:e
        >
          <li>
            <a
              class = "cursor-pointer"
              on:click=move |_| {
                let field_id = &e.field_id;
                let element_id = format!("#{field_id}");
                let element: web_sys::HtmlInputElement = document().query_selector(&element_id).unwrap().unwrap().unchecked_into();
                // uses might have to click the list link twice because if they are in input editing the on:blur event needs to change the html first and
                // this seems to interfere with this focus event
                before_focus();
                let _ = element.focus();
              }
            >
              { e.label }
            </a>
          </li>
        </For>
      </ul>
    }
}

#[derive(Debug, Clone)]
pub struct FieldSet<ID> {
    pub title: Option<&'static str>,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Clone)]
pub struct Field<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

impl<ID> Field<ID> {
    pub const fn unit(&self) -> Option<&'static str> {
        match self.field_type {
            FieldType::Float { unit, .. } => Some(unit),
            _ => None,
        }
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
        unit: &'static str,
    },
    UnsignedInteger {
        initial_value: Option<u64>,
        placeholder: Option<&'static str>,
        limits: MinMax<u64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_german_input_string_as_f64() {
        let result = parse_de_str_as_f64("1.100.100,23");
        assert_eq!(result, Ok(1_100_100.23));
    }
    #[test]
    fn parse_a_german_input_string_as_f64_trailing_space() {
        let result = parse_de_str_as_f64("1.100.100,23 ");
        assert_eq!(result, Ok(1_100_100.23));
    }
    #[test]
    fn parse_a_german_input_string_as_f64_leading_space() {
        let result = parse_de_str_as_f64(" 1.100.100,23");
        assert_eq!(result, Ok(1_100_100.23));
    }

    #[test]
    fn format_f64_as_german_string() {
        assert_eq!(
            format_f64_into_de_string(23_222_221_231.766_6),
            "23.222.221.231,7666"
        );
        assert_eq!(
            format_f64_into_de_string(23_222_221_231.0),
            "23.222.221.231"
        );
        assert_eq!(format_f64_into_de_string(2.0), "2");
    }
}
