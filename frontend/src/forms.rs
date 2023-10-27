use std::{
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use inflector::cases::kebabcase::to_kebab_case;
use leptos::*;

#[derive(Debug)]
pub struct Field<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

#[derive(Debug)]
pub enum FieldType {
    Float {
        initial_value: Option<f64>,
        placeholder: Option<&'static str>,
        min_value: Option<f64>,
        max_value: Option<f64>,
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

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn unique_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

impl<ID> Field<ID>
where
    ID: Copy + AsRef<str>,
{
    fn form_field_id(&self) -> String {
        // DOM element IDs needs to be locally unique
        // within the HTML document.
        let id = unique_id();

        // The name is only for humans for better readability.
        let name = to_kebab_case(self.id.as_ref());

        format!("{name}-{id}")
    }
}

#[derive(Debug)]
pub struct FieldSet<ID> {
    pub title: &'static str,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SelectOption {
    pub label: &'static str,
    pub value: usize,
}

#[derive(Clone, Copy)]
pub enum FieldSignal {
    Float(RwSignal<Option<f64>>),
    Text(RwSignal<Option<String>>),
    Bool(RwSignal<bool>),
    Selection(RwSignal<Option<usize>>),
}

impl FieldSignal {
    pub fn get_float(&self) -> Option<f64> {
        match self {
            Self::Float(s) => s.get(),
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

    pub fn get_selection(&self) -> Option<usize> {
        match self {
            Self::Selection(s) => s.get(),
            _ => None,
        }
    }

    pub fn get_float_signal(&self) -> Option<RwSignal<Option<f64>>> {
        match self {
            Self::Float(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get_text_signal(&self) -> Option<RwSignal<Option<String>>> {
        match self {
            Self::Text(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get_bool_signal(&self) -> Option<RwSignal<bool>> {
        match self {
            Self::Bool(s) => Some(*s),
            _ => None,
        }
    }

    pub fn get_selection_signal(&self) -> Option<RwSignal<Option<usize>>> {
        match self {
            Self::Selection(s) => Some(*s),
            _ => None,
        }
    }

    pub fn clear(&self) {
        match self {
            Self::Float(s) => s.set(None),
            Self::Text(s) => s.set(None),
            Self::Bool(s) => s.set(false),
            Self::Selection(s) => s.set(None),
        }
    }
}

pub fn render_field_sets<ID>(
    field_sets: Vec<FieldSet<ID>>,
) -> (HashMap<ID, FieldSignal>, Vec<impl IntoView>)
where
    ID: AsRef<str> + Copy + Hash + Eq,
{
    let mut signals = HashMap::new();
    let mut set_views = vec![];

    for set in field_sets {
        let mut field_views = vec![];

        for field in set.fields {
            let id = field.id;
            let (field_signal, view) = render_field(field);
            field_views.push(view);
            signals.insert(id, field_signal);
        }

        set_views.push(
            view! {
              <div class="border-b border-gray-900/10 pb-12">
                <h3 class="text-lg font-semibold leading-7 text-gray-900">{ set.title }</h3>
                <div class="mt-10 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-4">
                  { field_views }
                </div>
              </div>
            }
            .into_view(),
        );
    }
    (signals, set_views)
}

fn render_field<ID>(field: Field<ID>) -> (FieldSignal, impl IntoView)
where
    ID: AsRef<str> + Copy,
{
    let Field {
        description, label, ..
    } = field;

    let field_id = field.form_field_id();

    match field.field_type {
        FieldType::Text {
            placeholder,
            initial_value,
            max_len,
        } => {
            let signal = create_rw_signal(initial_value);
            let field_signal = FieldSignal::Text(signal);
            let view = view! {
              <TextInput
                label
                field_id
                placeholder = placeholder.unwrap()
                value = signal
                max_len
              />
            }
            .into_view();
            (field_signal, view)
        }
        FieldType::Float {
            placeholder,
            unit,
            initial_value,
            ..
        } => {
            let signal = create_rw_signal(initial_value);
            let field_signal = FieldSignal::Float(signal);

            let view = view! {
              <NumberInput
                label
                field_id
                placeholder = placeholder.unwrap()
                value = signal
                unit
              />
            }
            .into_view();
            (field_signal, view)
        }
        FieldType::Bool { initial_value } => {
            let signal = create_rw_signal(initial_value.unwrap_or_default());
            let field_signal = FieldSignal::Bool(signal);
            let view = view! {
              <BoolInput
                label
                field_id
                value = signal
                comment = description
              />
            }
            .into_view();
            (field_signal, view)
        }
        FieldType::Selection {
            initial_value,
            options,
        } => {
            let signal = create_rw_signal(initial_value);
            let field_signal = FieldSignal::Selection(signal);
            let view = view! {
              <SelectInput
                label
                field_id
                value = signal
                options
              />
            }
            .into_view();
            (field_signal, view)
        }
    }
}

#[component]
fn TextInput(
    label: &'static str,
    field_id: String,
    placeholder: &'static str,
    value: RwSignal<Option<String>>,
    max_len: Option<usize>,
) -> impl IntoView {
    view! {
      <div>
        <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            type = "text"
            id = { field_id }
            maxlength = { max_len }
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            prop:value = move || value.get().unwrap_or_default()
            on:input = move |ev| {
              let target_value = event_target_value(&ev);
              if target_value.is_empty() {
                value.set(None);
              } else  {
                value.set(Some(target_value));
              }
            }
          />
        </div>
      </div>
    }
}

#[component]
fn NumberInput(
    label: &'static str,
    unit: &'static str,
    placeholder: &'static str,
    field_id: String,
    value: RwSignal<Option<f64>>,
) -> impl IntoView {

    let l10n = L10n::De;
    let initial_output = format_number_input(value.get(), l10n);
    let input = RwSignal::new(initial_output);
    let error = RwSignal::new(Option::<String>::None);
    let is_editing = RwSignal::new(false);

    view! {
      <div>
        <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { field_id }
            type="text"
            class = move || {
              let bg = if error.get().is_some() { "bg-red-100" } else { "" };
              format!("{bg} block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6")
            }
            placeholder= { placeholder }
            // TODO: aria-describedby
            prop:value = move ||
              if is_editing.get() {
                input.get()
              } else {
                format_number_input(value.get(), l10n)
            }
            on:focus = move |_| {
              log::debug!("start editing");
              is_editing.set(true);
            }
            on:keyup = move |ev| {
              if ev.key() == "Enter" {
                // TODO: emit blur event
                input.set(format_number_input(value.get(), l10n));
                is_editing.set(false);
              }
            }
            on:input = move |ev| {
              let target_value = event_target_value(&ev);
              match parse_number_input_as_float(&target_value, l10n) {
                Ok(v) => {
                  value.set(v);
                  error.set(None);
                  input.set(target_value);
                }
                Err(err) => {
                  let error_message = format!("{err}"); // TODO
                  error.set(Some(error_message));
                }
              }
            }
            on:blur = move |_| {
              log::debug!("Stop editing");
              input.set(format_number_input(value.get(), l10n));
              is_editing.set(false);
            }
          />
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
            <span class="text-gray-500 sm:text-sm">{ unit }</span>
          </div>
        </div>
      </div>
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum L10n {
  Us,
  De
}

impl L10n {
  const fn thousands_separator(&self) -> &str {
      match self {
          Self::Us => ",",
          Self::De => "."
      }
  }
  const fn decimal_point(&self) -> &str {
      match self {
          Self::Us => ".",
          Self::De => ","
      }
  }
}

fn parse_number_input_as_float(input: &str, l10n: L10n) -> anyhow::Result<Option<f64>> {
   let input = input.trim();
   if input.trim().is_empty() {
     return Ok(None);
   }

   let input = if let Some(',') = input.chars().last() {
      &input[..input.len()-1]
   } else {
      input
   };

   let input = input
      .replace(l10n.thousands_separator(),"")
      .replace(',',".");

   Ok(Some(input.parse()?))
}

fn format_number_input(value: Option<f64>, l10n: L10n) -> String {
  match value {
    Some(v) => {
      v.to_string().replace(".",l10n.decimal_point()) // TODO: add thousands separator
    }
    None => String::new()
  }
}

#[test]
fn test_parse_number_input_as_float() {
  assert_eq!(parse_number_input_as_float("", L10n::De).unwrap(), None);
  assert_eq!(parse_number_input_as_float("1", L10n::De).unwrap(), Some(1.0));
  assert_eq!(parse_number_input_as_float("1,", L10n::De).unwrap(), Some(1.0));
  assert_eq!(parse_number_input_as_float("1,2", L10n::De).unwrap(), Some(1.2));
  assert_eq!(parse_number_input_as_float("1.000,", L10n::De).unwrap(), Some(1000.0));
  assert_eq!(parse_number_input_as_float("1.00,", L10n::De).unwrap(), Some(100.0));
  assert_eq!(parse_number_input_as_float("1.0", L10n::De).unwrap(), Some(10.0));
}

#[component]
fn BoolInput(
    label: &'static str,
    field_id: String,
    value: RwSignal<bool>,
    comment: Option<&'static str>,
) -> impl IntoView {
    view! {
      <div class="relative flex items-start">
        <div class="flex h-6 items-center">
          <input
            id = { &field_id }
            type="checkbox"
            class="h-4 w-4 rounded border-gray-300 text-highlight focus:ring-highlight"
            // TODO: aria-describedby
            prop:checked = move || value.get()
            on:input = move |_| { value.update(|v| *v = !*v); }
          />
        </div>
        <div class="ml-3 text-sm leading-6">
          <label for={ field_id } class="font-bold text-gray-900">{ label }</label>
          <p class="text-gray-500">{ comment }</p>
        </div>
      </div>
    }
}

#[component]
fn SelectInput(
    label: &'static str,
    field_id: String,
    value: RwSignal<Option<usize>>,
    options: Vec<SelectOption>,
) -> impl IntoView {
    view! {
      <div>
        <label
          for={ &field_id }
          class="block text-sm font-bold leading-6 text-gray-900"
        >
          { label }
        </label>
        <select
          id = { field_id }
          class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
          on:change = move |ev| {
              let target_value = event_target_value(&ev);
              if target_value.is_empty() {
                value.set(None);
              } else {
                 match target_value.parse() {
                    Ok(v) => { value.set(Some(v)) },
                    Err(_) => {
                      value.set(None);
                      log::error!("Unexpected option value {target_value}");
                    },
                 }
              }
          }
        >
          <Options value options />
        </select>
      </div>
    }
}

#[component]
fn Options(value: RwSignal<Option<usize>>, options: Vec<SelectOption>) -> impl IntoView {
    view! {
      <option prop:selected = move || value.get().is_none()>
        " - Bitte wählen - "
      </option>
      <For
        each = move || options.clone()
        key = |option| option.value
        let:option
      >
        <option
          value = option.value
          prop:selected = move || value.get() == Some(option.value)
        >
          { option.label }
        </option>
      </For>
    }
}
