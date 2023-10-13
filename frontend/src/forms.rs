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
    view! {
      <div>
        <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { field_id }
            type="text"
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            prop:value = move || value.get().map(|v|v.to_string().replace('.',",")).unwrap_or_default()
            on:input = move |ev| {
              let target_value = event_target_value(&ev);
              if target_value.is_empty() {
                value.set(None);
              } else if let Ok(target_value) = target_value.replace(',',".").parse() {
                value.set(Some(target_value));
              }
            }
          />
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
            <span class="text-gray-500 sm:text-sm">{ unit }</span>
          </div>
        </div>
      </div>
    }
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
