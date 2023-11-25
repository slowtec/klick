use std::{
    collections::HashMap,
    fmt::Write,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use inflector::cases::kebabcase::to_kebab_case;
use leptos::*;

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

    pub const fn get_float_signal(&self) -> Option<RwSignal<Option<String>>> {
        // FIXME rename
        match self {
            Self::Float { input, .. } => Some(*input),
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

    pub fn clear(&self) {
        match self {
            Self::Float { input, .. } => input.set(None),
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
                <h3 class="text-lg font-semibold leading-7 text-gray-900">
                  { set.title }
                </h3>
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
        description,
        label,
        required,
        ..
    } = field;

    let field_id = crate::forms::form_field_id(&field.id);

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
                placeholder = placeholder.unwrap_or_default()
                value = signal
                max_len
                description
                required
              />
            }
            .into_view();
            (field_signal, view)
        }
        FieldType::Float {
            placeholder,
            unit,
            initial_value,
            limits,
            ..
        } => {
            let i_value = initial_value.map(format_f64_into_de_string);

            let input_signal = RwSignal::new(i_value);
            let output_signal = RwSignal::new(Option::<f64>::None);

            let field_signal = FieldSignal::Float {
                input: input_signal,
                output: output_signal,
            };

            let view = view! {
              <NumberInput
                label
                field_id
                placeholder = placeholder.unwrap_or_default()
                input_value = input_signal
                output_value = output_signal
                unit
                description
                limits
                required
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
                description
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

fn create_tooltip(
    label: &'static str,
    description: Option<&'static str>,
    required: bool,
    _unit: Option<&'static str>,
    limits: Option<MinMax>,
) -> impl IntoView {
    let show_tooltip = RwSignal::new("none".to_string());

    view! {
      <div class="flex-col md:flex-row flex items-center md:justify-center">
        <a
          tabindex="-1"
          role="link"
          aria-label="tooltip 1"
          class="focus:outline-none focus:ring-gray-300 rounded-full focus:ring-offset-2 focus:ring-2 focus:bg-gray-200 relative mt-20 md:mt-0"
          on:focus = move |_| {
              show_tooltip.set("block".to_string());
          }
          on:blur = move |_| {
              show_tooltip.set("none".to_string());
          }
        >
          <div class="cursor-pointer">
            <svg
              aria-haspopup="true"
              class="icon icon-tabler icon-tabler-info-circle"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="#A0AEC0"
              fill="none"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path stroke="none" d="M0 0h24v24H0z" />
              <circle cx="12" cy="12" r="9" />
              <line x1="12" y1="8" x2="12.01" y2="8" />
              <polyline points="11 12 12 12 12 16 13 16" />
            </svg>
          </div>
          <div
            role="tooltip"
            class="z-20 -mt-20 w-64 absolute transition duration-150 ease-in-out left-0 ml-8 shadow-lg bg-white p-4 rounded"
            style={ move || format!("display: {}", show_tooltip.get()) }
          >
            <svg
              class="absolute left-0 -ml-2 bottom-0 top-0 h-full"
              width="9px"
              height="16px"
              viewBox="0 0 9 16"
            >
              <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                <g transform="translate(-874.000000, -1029.000000)" fill="#FFFFFF">
                  <g transform="translate(850.000000, 975.000000)">
                    <g transform="translate(24.000000, 0.000000)">
                      <polygon
                        transform="translate(4.500000, 62.000000) rotate(-90.000000) translate(-4.500000, -62.000000)"
                        points="4.5 57.5 12.5 66.5 -3.5 66.5">
                      </polygon>
                    </g>
                  </g>
                </g>
              </g>
            </svg>
            <p class="text-sm font-bold text-gray-800 pb-1">{ label }</p>
            <p class="text-xs leading-4 text-gray-600 pb-3">{ description }</p>
            <Show when=move || (limits.is_some() || limits.is_some() || required )>
              <ul class="list-disc list-inside">
              <Show when=move || required>
                <li class="text-xs leading-4 text-gray-600 pb-3">"Eingabe benötigt!"</li>
              </Show>
              </ul>
            </Show>
          </div>
        </a>
      </div>
    }
}

#[component]
fn TextInput(
    label: &'static str,
    field_id: String,
    placeholder: &'static str,
    value: RwSignal<Option<String>>,
    max_len: Option<usize>,
    description: Option<&'static str>,
    required: bool,
) -> impl IntoView {
    let required_label = format!("{} {label}", if required { "*" } else { "" });
    view! {
      <div>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">
            { required_label }
          </label>
          { create_tooltip(label, description, required, None, None) }
        </div>

        <div class="relative mt-2 rounded-md shadow-sm group">
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
              let v = if target_value.is_empty() { None } else { Some(target_value) };
              value.set(v);
            }
          />
        </div>
      </div>
    }
}

fn parse_de_str_as_f64(input: &str) -> Result<f64, String> {
    let float = input.replace('.', "").replace(',', ".").parse::<f64>();
    match float {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e}")),
    }
}

pub fn format_f64_into_de_string(number: f64) -> String {
    let num_str = format!("{number:.}");

    let (integer, decimal) = if let Some(pos) = num_str.find('.') {
        num_str.split_at(pos)
    } else {
        (&*num_str, "")
    };

    let integer_string =
        integer
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |mut output, (i, c)| {
                let z: &str = if i != 0 && i % 3 == 0 && i != integer.len() {
                    "."
                } else {
                    ""
                };
                let _ = write!(output, "{z}{c}");
                output
            });

    let v = integer_string.chars().rev().collect::<String>();
    format!("{v}{}", decimal.replace('.', ","))
}

#[component]
fn NumberInput(
    label: &'static str,
    unit: &'static str,
    placeholder: &'static str,
    field_id: String,
    input_value: RwSignal<Option<String>>,
    output_value: RwSignal<Option<f64>>,
    description: Option<&'static str>,
    limits: MinMax,
    required: bool,
) -> impl IntoView {
    let required_label = format!("{} {}", if required { "*" } else { "" }, label);
    let error = RwSignal::new(Option::<String>::None);

    view! {
      <div>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">
            { required_label }
          </label>
          { create_tooltip(label, description, required, Some(unit), Some(limits)) }
        </div>

        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { field_id }
            type="text"
            class = move || {
              let bg = if error.get().is_some() { "bg-red-100" } else { "" };
              format!("{} {bg}", "block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6")
            }
            placeholder = { placeholder }
            on:focusin = move |_ev| {
              if let Some(v) = input_value.get() {
                  let v = v.replace('.', "");
                  input_value.set(Some(v));
              }
            }
            on:focusout = move |_ev| {
                let Some(v) = input_value.get() else {
                    input_value.set(None);
                    if required {
                        error.set(Some("Eingabe benötigt!".to_string()));
                    }
                    return;
                };
                if let Ok(q) = parse_de_str_as_f64(&v) {
                  let s = format_f64_into_de_string(q);
                  input_value.set(Some(s));
                }
            }
            prop:value = move || {
                let Some(v) = input_value.get() else {
                    // do not reset error because on:focusout could have set it
                    output_value.set(None);
                    return String::new();
                };
                let Ok(t) = parse_de_str_as_f64(&v) else {
                    error.set(Some("Fehlerhafte Eingabe!".to_string()));
                    output_value.set(None);
                    return v;
                };
                if let Some(min) = limits.min {
                    if t <= min {
                        error.set(Some("Eingabe unterschreitet das Minimum".to_string()));
                        output_value.set(None);
                        return v;
                    }
                }
                if let Some(max) = limits.max {
                    if t >= max {
                        error.set(Some("Eingabe überschreitet das Maximum".to_string()));
                        output_value.set(None);
                        return v;
                    }
                }
                error.set(None);
                output_value.set(Some(t));
                v
            }
            on:input = move |ev| {
              let input = event_target_value(&ev);
              let v = if input.is_empty() { None } else { Some(input) };
              input_value.set(v);
            }
          />
          <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
            <span class="text-gray-500 sm:text-sm">{ unit }</span>
          </div>
        </div>
        <Show when=move || error.get().is_some()>
          <p class="mt-2 text-sm" style="color: red">{ move || error.get() }</p>
        </Show>
      </div>
    }
}

#[component]
fn BoolInput(
    label: &'static str,
    field_id: String,
    value: RwSignal<bool>,
    description: Option<&'static str>,
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
          <p class="text-gray-500">{ description }</p>
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
              } else if let Ok(v) = target_value.parse() {
                  value.set(Some(v));
              } else {
                  value.set(None);
                  log::error!("Unexpected option value {target_value}");
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

#[derive(Debug)]
pub struct FieldSet<ID> {
    pub title: &'static str,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug)]
pub struct Field<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
    pub field_type: FieldType,
}

#[derive(Debug, Clone, Copy)]
pub struct MinMax {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug)]
#[allow(dead_code)]
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
