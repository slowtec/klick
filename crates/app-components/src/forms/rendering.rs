use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
};

use leptos::*;
use thiserror::Error;

use super::{Field, FieldId, FieldSet, FieldType, MinMax};

type MissingFields = HashSet<FieldId>;
type Labels = HashMap<FieldId, &'static str>;

pub fn render_field_sets(
    field_sets: Vec<FieldSet>,
) -> (Vec<View>, ReadSignal<MissingFields>, Labels) {
    let mut set_views = vec![];
    let mut labels = HashMap::new();
    let missing_fields = RwSignal::new(HashSet::new());

    for set in field_sets {
        let mut field_views = vec![];

        for field in set.fields {
            let id = crate::forms::dom_node_id();
            labels.insert(id, field.label);
            let view = render_field(field, id, missing_fields);
            field_views.push(view);
        }

        set_views.push(
            view! {
              <fieldset class="border-b border-gray-900/10 pb-12 mb-6">
                {
                  set.title.map(|title| view! {
                    <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">
                      { title }
                    </h3>
                  })
                }
                <div class="mt-6 grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-4">
                  { field_views }
                </div>
              </fieldset>
            }
            .into_view(),
        );
    }
    (set_views, missing_fields.read_only(), labels)
}

pub fn render_field(
    field: Field,
    id: FieldId,
    missing_fields: RwSignal<HashSet<FieldId>>,
) -> impl IntoView {
    let Field {
        label,
        description,
        required,
        ..
    } = field;

    match field.field_type {
        FieldType::Text {
            placeholder,
            initial_value,
            max_len,
            on_change,
            input,
        } => {
            if required && initial_value.is_none() {
                missing_fields.update(|x| {
                    x.insert(id);
                });
            }

            if required {
                create_effect(move |_| {
                    if input.with(|x| x.is_some()) {
                        missing_fields.update(|x| {
                            x.remove(&id);
                        });
                    } else {
                        missing_fields.update(|x| {
                            x.insert(id);
                        });
                    }
                });
            }
            let view = view! {
              <TextInput
                label
                id
                placeholder = placeholder.unwrap_or_default()
                max_len
                description
                required
                input_value = input
                on_change = on_change
              />
            }
            .into_view();
            view
        }
        FieldType::Float {
            placeholder,
            unit,
            initial_value,
            limits,
            on_change,
            input,
            ..
        } => {
            // TODO: write initial value

            if required && initial_value.is_none() {
                missing_fields.update(|x| {
                    x.insert(id);
                });
            }
            if required {
                create_effect(move |_| {
                    if input.with(|x| x.is_some()) {
                        missing_fields.update(|x| {
                            x.remove(&id);
                        });
                    } else {
                        missing_fields.update(|x| {
                            x.insert(id);
                        });
                    }
                });
            }

            view! {
              <FloatInput
                label
                id
                placeholder = placeholder.unwrap_or_default()
                unit
                description
                limits
                required
                input_value = input
                on_change
              />
            }
            .into_view()
        }
        FieldType::UnsignedInteger {
            placeholder,
            unit,
            initial_value,
            limits,
            on_change,
            input,
            ..
        } => {
            if required && initial_value.is_none() {
                missing_fields.update(|x| {
                    x.insert(id);
                });
            }
            if required {
                create_effect(move |_| {
                    if input.get().is_some() {
                        missing_fields.update(|x| {
                            x.remove(&id);
                        });
                    } else {
                        missing_fields.update(|x| {
                            x.insert(id);
                        });
                    }
                });
            }

            let view = view! {
              <UnsignedIntegerInput
                label
                id
                placeholder = placeholder.unwrap_or_default()
                unit
                description
                limits
                required
                input_value = input
                on_change
              />
            }
            .into_view();
            view
        }
        FieldType::Bool {
            initial_value: _,
            on_change,
            input,
        } => {
            let view = view! {
              <BoolInput
                label
                id
                input_value = input.into()
                description
                on_change
              />
            }
            .into_view();
            view
        }
    }
}

pub use crate::icons::InformationCircle as InfoIcon;

// TODO: don't render if description is None
fn create_tooltip(
    label: &'static str,
    description: Option<&'static str>,
    required: bool,
    _unit: Option<&'static str>,
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
            <InfoIcon />
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
            <p class="text-xs leading-4 text-gray-600 pb-3" inner_html=description/>
            <Show when=move || required>
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
    id: FieldId,
    placeholder: String,
    max_len: Option<usize>,
    description: Option<&'static str>,
    required: bool,
    input_value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>, ()>,
) -> impl IntoView {
    let required_label = format!("{} {label}", if required { "*" } else { "" });

    view! {
      <div id={format!("focus-{id}")}>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ id.to_string() } class="block text-sm font-bold leading-6 text-gray-900">
            { required_label }
          </label>
          { create_tooltip(label, description, required, None) }
        </div>

        <div class="relative mt-2 rounded-md shadow-sm group">
          <input
            type = "text"
            id = { id.to_string() }
            maxlength = { max_len }
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            prop:value = move || {
                input_value.get().unwrap_or_default()
            }
            on:input = move |ev| {
              let target_value = event_target_value(&ev);
              let v = if target_value.is_empty() { None } else { Some(target_value) };
              on_change.call(v);
            }
            // FIXME: trigger focusout on Enter or Escape
          />
        </div>
      </div>
    }
}

pub fn parse_de_str_as_f64(input: &str) -> Result<f64, String> {
    let float = input
        .replace('.', "")
        .replace(',', ".")
        .trim()
        .parse::<f64>();
    match float {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("{e}")),
    }
}

// TODO: move this out of this layer (e.g. to the  presenter)
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
fn FloatInput(
    label: &'static str,
    unit: &'static str,
    placeholder: String,
    id: FieldId,
    description: Option<&'static str>,
    limits: MinMax<f64>,
    required: bool,
    input_value: Signal<Option<f64>>,
    #[prop(into)] on_change: Callback<Option<f64>, ()>,
) -> impl IntoView {
    let required_label = format!("{} {}", if required { "*" } else { "" }, label);
    let error = RwSignal::new(Option::<String>::None);
    let txt = RwSignal::new(String::new());
    let is_focussed = RwSignal::new(false);

    create_effect(move |_| {
        if !is_focussed.get() {
            txt.set(
                input_value
                    .get()
                    .map(format_f64_into_de_string)
                    .unwrap_or_else(String::new),
            );
        }
        match evaluate_float_input(&txt.get(), required, limits) {
            Ok(_) => {
                error.set(None);
            }
            Err(err) => {
                if err == FloatEvalError::Empty && !required {
                    error.set(None);
                }
            }
        }
    });

    let on_input = move |ev| {
        let input = event_target_value(&ev);
        let result = evaluate_float_input(&input, required, limits);
        txt.set(input);
        match result {
            Err(err) => {
                error.set(Some(err.to_string()));
                if err == FloatEvalError::Empty {
                    on_change.call(None);
                }
            }
            Ok(value) => {
                error.set(None);
                on_change.call(value);
            }
        }
    };

    let on_focus_out = move |_| {
        is_focussed.set(false);
    };
    let on_focus_in = move |_| {
        is_focussed.set(true);
    };

    // FIXME: Format input as string and remove delemiters
    // FIXME: trigger focusout on Enter or Escape

    view! {
      <div>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ id.to_string() } class="block text-sm font-bold leading-6 text-gray-900">
            { required_label }
          </label>
          { create_tooltip(label, description, required, Some(unit)) }
        </div>

        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { id.to_string() }
            type="text"
            class = move || {
              let bg = if error.get().is_some() { "bg-red-100" } else { "" };
              format!("{} {bg}", "block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6")
            }
            placeholder = { placeholder }
            on:input = on_input
            on:focusin = on_focus_in
            on:focusout = on_focus_out
            prop:value = txt
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

type FloatEvalError = NumberEvalError<f64>;

#[derive(Debug, PartialEq, Clone, Copy, Error)]
enum NumberEvalError<T> {
    #[error("Eingabe benötigt!")]
    Empty,
    #[error("Fehlerhafte Eingabe!")]
    Invalid,
    #[error("Eingabe unterschreitet das Minimum ({0})")]
    TooSmall(T),
    #[error("Eingabe überschreitet das Maximum ({0})")]
    TooBig(T),
}

fn evaluate_float_input(
    txt: &str,
    required: bool,
    limits: MinMax<f64>,
) -> Result<Option<f64>, FloatEvalError> {
    evaluate_number_input(txt, required, limits, |x| parse_de_str_as_f64(x).ok())
}

fn evaluate_u64_input(
    txt: &str,
    required: bool,
    limits: MinMax<u64>,
) -> Result<Option<u64>, NumberEvalError<u64>> {
    evaluate_number_input(txt, required, limits, |x| x.parse().ok())
}

fn evaluate_number_input<T, F>(
    txt: &str,
    required: bool,
    limits: MinMax<T>,
    parse: F,
) -> Result<Option<T>, NumberEvalError<T>>
where
    T: PartialOrd + Copy,
    F: Fn(&str) -> Option<T>,
{
    if txt.is_empty() {
        if required {
            return Err(NumberEvalError::Empty);
        } else {
            return Ok(None);
        }
    }
    let Some(v) = parse(txt) else {
        return Err(NumberEvalError::Invalid);
    };
    check_min_max(v, limits)?;
    Ok(Some(v))
}

fn check_min_max<T: PartialOrd>(v: T, limits: MinMax<T>) -> Result<(), NumberEvalError<T>> {
    if let Some(min) = limits.min {
        if v < min {
            return Err(NumberEvalError::TooSmall(min));
        }
    }
    if let Some(max) = limits.max {
        if v > max {
            return Err(NumberEvalError::TooBig(max));
        }
    }
    Ok(())
}

#[component]
fn UnsignedIntegerInput(
    label: &'static str,
    unit: &'static str,
    placeholder: String,
    id: FieldId,
    description: Option<&'static str>,
    limits: MinMax<u64>,
    required: bool,
    input_value: Signal<Option<u64>>,
    #[prop(into)] on_change: Callback<Option<u64>, ()>,
) -> impl IntoView {
    let required_label = format!("{} {}", if required { "*" } else { "" }, label);
    let error = RwSignal::new(Option::<String>::None);
    let txt = RwSignal::new(String::new());
    let is_focussed = RwSignal::new(false);

    create_effect(move |_| {
        if !is_focussed.get() {
            txt.set(
                input_value
                    .get()
                    .map(|v| format!("{v}"))
                    .unwrap_or_else(String::new),
            );
        }
        match evaluate_u64_input(&txt.get(), required, limits) {
            Ok(_) => {
                error.set(None);
            }
            Err(err) => {
                if err == NumberEvalError::Empty && !required {
                    error.set(None);
                }
            }
        }
    });

    let on_input = move |ev| {
        let input = event_target_value(&ev);
        let result = evaluate_u64_input(&input, required, limits);
        txt.set(input);
        match result {
            Err(err) => {
                error.set(Some(err.to_string()));
                if err == NumberEvalError::Empty && !required {
                    on_change.call(None);
                }
            }
            Ok(value) => {
                error.set(None);
                on_change.call(value);
            }
        }
    };

    let on_focus_out = move |_| {
        is_focussed.set(false);
    };
    let on_focus_in = move |_| {
        is_focussed.set(true);
    };

    // FIXME: Format input as string and remove delemiters
    // FIXME: trigger focusout on Enter or Escape

    view! {
      <div>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ id.to_string() } class="block text-sm font-bold leading-6 text-gray-900">
            { required_label }
          </label>
          { create_tooltip(label, description, required, Some(unit)) }
        </div>

        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { id.to_string() }
            type="text"
            class = move || {
              let bg = if error.get().is_some() { "bg-red-100" } else { "" };
              format!("{} {bg}", "block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6")
            }
            placeholder = { placeholder }
            on:input = on_input
            on:focusin = on_focus_in
            on:focusout = on_focus_out
            prop:value = txt
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
    id: FieldId,
    input_value: Signal<bool>,
    description: Option<&'static str>,
    #[prop(into)] on_change: Callback<bool, ()>,
) -> impl IntoView {
    view! {
      <div class="relative flex items-start">
        <div class="flex h-6 items-center">
          <input
            id = { id.to_string() }
            type="checkbox"
            class="h-4 w-4 rounded border-gray-300 text-highlight focus:ring-highlight"
            // TODO: aria-describedby
            prop:checked = move || input_value.get()
            on:input = move |_| {
                let v = !input_value.get();
                on_change.call(v);
            }
          />
        </div>
        <div class="ml-3 text-sm leading-6">
          <label for={ id.to_string() } class="font-bold text-gray-900">{ label }</label>
          <p class="text-gray-500">{ description }</p>
        </div>
      </div>
    }
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
