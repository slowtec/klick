use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use leptos::*;
use thiserror::Error;

use klick_presenter::Lng;

use crate::icons;

use super::{Field, FieldId, FieldSet, FieldType, MinMax};

type MissingFields = HashSet<FieldId>;
type Labels = HashMap<FieldId, Signal<String>>;

#[must_use]
pub fn render_field_sets(
    field_sets: Vec<FieldSet>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> (Vec<View>, ReadSignal<MissingFields>, Labels) {
    let mut set_views = vec![];
    let mut labels = HashMap::new();
    let missing_fields = RwSignal::new(HashSet::new());
    let lng = Lng::De; // TODO

    for set in field_sets {
        let mut field_views = vec![];

        for field in set.fields {
            let id = crate::forms::dom_node_id();
            labels.insert(id, field.label.clone());
            let view = render_field(
                field,
                id,
                missing_fields,
                lng,
                accessibility_always_show_option,
            );
            field_views.push(view);
        }

        let view = view! {
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
        .into_view();
        set_views.push(view);
    }
    (set_views, missing_fields.read_only(), labels)
}

#[must_use]
pub fn render_field(
    field: Field,
    id: FieldId,
    missing_fields: RwSignal<HashSet<FieldId>>,
    lng: Lng,
    accessibility_always_show_option: Option<RwSignal<bool>>,
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
                Effect::new(move |_| {
                    if input.with(std::option::Option::is_some) {
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
              <TextInput
                label
                id
                placeholder = placeholder.unwrap_or_default()
                max_len
                description
                required
                input_value = input
                on_change = on_change
                accessibility_always_show_option
              />
            }
            .into_view()
        }
        FieldType::Float {
            placeholder,
            unit,
            initial_value,
            limits,
            on_change,
            input,
        } => {
            // TODO: write initial value

            if required && initial_value.is_none() {
                missing_fields.update(|x| {
                    x.insert(id);
                });
            }
            if required {
                Effect::new(move |_| {
                    if input.with(std::option::Option::is_some) {
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
                lng
                accessibility_always_show_option
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
        } => {
            if required && initial_value.is_none() {
                missing_fields.update(|x| {
                    x.insert(id);
                });
            }
            if required {
                Effect::new(move |_| {
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

            view! {
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
                lng
                accessibility_always_show_option
              />
            }
            .into_view()
        }
        FieldType::Bool {
            initial_value: _,
            on_change,
            input,
        } => view! {
          <BoolInput
            label
            id
            input_value = input
            description
            on_change
          />
        }
        .into_view(),
    }
}

fn create_tooltip(
    label: Signal<String>,
    description: Option<&'static str>, // TODO: pass description as Markdown instead of raw HTML
    required: bool,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    id: FieldId,
) -> impl IntoView {
    let Some(description) = description else {
        return view! {}.into_view();
    };

    let show_tooltip = RwSignal::new(false);
    let required_hint = if required { "* " } else { "" };
    let always_show_accessibility = accessibility_always_show_option.is_some_and(|o| o.get());

    // TODO: check current language
    let input_required_message = "Eingabe benötigt!";
    let required_msg = view! {
      <Show when=move || required>
        <ul class="list-disc list-inside">
          <li class="text-xs leading-4 text-gray-600 pb-3">
            { input_required_message }
          </li>
        </ul>
      </Show>
    };

    if always_show_accessibility {
        return view! {
          <div>
            <label for={ id.to_string() } class="block text-sm font-bold leading-6 text-gray-900">
              { required_hint }
              { label }
            </label>
            <p class="text-xs leading-4 text-gray-600 pb-3" inner_html=description/>
            { required_msg }
          </div>
        }
        .into_view();
    }

    view! {
      <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
        <label for={ id.to_string() } class="block text-sm font-bold leading-6 text-gray-900">
          { required_hint }
          { label.clone() }
        </label>
        <div class="flex-col md:flex-row flex items-center md:justify-center">
          <a
            tabindex="-1"
            role="link"
            aria-label="tooltip 1"
            class="focus:outline-none focus:ring-gray-300 rounded-full focus:ring-offset-2 focus:ring-2 focus:bg-gray-200 relative mt-20 md:mt-0"
            on:focus = move |_| {
                show_tooltip.set(true);
            }
            on:blur = move |_| {
                show_tooltip.set(false);
            }
          >
            <div class="cursor-pointer">
              <icons::InformationCircle />
            </div>
            <div
              role="tooltip"
              class="z-20 -mt-20 w-64 absolute transition duration-150 ease-in-out left-0 ml-8 shadow-lg bg-white p-4 rounded"
              style:display= move || if show_tooltip.get() { "block" } else { "none" }
            >
              <TriangleIcon />
              <p class="text-sm font-bold text-gray-800 pb-1">{ label }</p>
              <p class="text-xs leading-4 text-gray-600 pb-3" inner_html=description />
              { required_msg }
            </div>
          </a>
        </div>
      </div>
    }.into_view()
}

#[component]
fn TriangleIcon() -> impl IntoView {
    view! {
      <svg
        class="absolute left-0 -ml-2 bottom-0 top-0 h-full"
        width="9px"
        height="16px"
        viewBox="0 0 9 16"
      >
        <g stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
          <g transform="translate(-874.0, -1029.0)" fill="#FFFFFF">
            <g transform="translate(850.0, 975.0)">
              <g transform="translate(24.0, 0.0)">
                <polygon
                  transform="translate(4.5, 62.0) rotate(-90.0) translate(-4.5, -62.0)"
                  points="4.5 57.5 12.5 66.5 -3.5 66.5">
                </polygon>
              </g>
            </g>
          </g>
        </g>
      </svg>
    }
}

#[component]
fn TextInput(
    label: Signal<String>,
    id: FieldId,
    placeholder: Signal<String>,
    max_len: Option<usize>,
    description: Option<&'static str>,
    required: bool,
    input_value: Signal<Option<String>>,
    #[prop(into)] on_change: Callback<Option<String>, ()>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
      <div id={ format!("focus-{id}") }>
        { move || create_tooltip(label.clone(), description, required, accessibility_always_show_option, id) }
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

#[component]
fn FloatInput(
    label: Signal<String>,
    unit: &'static str,
    placeholder: Signal<String>,
    id: FieldId,
    description: Option<&'static str>,
    limits: MinMax<f64>,
    required: bool,
    input_value: Signal<Option<f64>>,
    #[prop(into)] on_change: Callback<Option<f64>, ()>,
    lng: Lng,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let format_number = move |v| lng.format_number(v);
    let format_plain_number = move |v| lng.format_number_without_thousands_separators(v);
    number_input_field(
        label,
        unit,
        placeholder,
        id,
        description,
        limits,
        required,
        input_value,
        on_change,
        evaluate_float_input,
        lng,
        format_number,
        format_plain_number,
        accessibility_always_show_option,
    )
}

#[component]
fn UnsignedIntegerInput(
    label: Signal<String>,
    unit: &'static str,
    placeholder: Signal<String>,
    id: FieldId,
    description: Option<&'static str>,
    limits: MinMax<u64>,
    required: bool,
    input_value: Signal<Option<u64>>,
    #[prop(into)] on_change: Callback<Option<u64>, ()>,
    lng: Lng,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let format_number = move |v| lng.format_number(v as f64);
    let format_plain_number = move |v| lng.format_number_without_thousands_separators(v as f64);
    number_input_field(
        label,
        unit,
        placeholder,
        id,
        description,
        limits,
        required,
        input_value,
        on_change,
        evaluate_u64_input,
        lng,
        format_number,
        format_plain_number,
        accessibility_always_show_option,
    )
}

fn number_input_field<F, N>(
    label: Signal<String>,
    unit: &'static str,
    placeholder: Signal<String>,
    id: FieldId,
    description: Option<&'static str>,
    limits: MinMax<N>,
    required: bool,
    input_value: Signal<Option<N>>,
    on_change: Callback<Option<N>, ()>,
    evaluate_input: F,
    lng: Lng,
    format_value: impl Fn(N) -> String + Copy + 'static,
    format_plain_value: impl Fn(N) -> String + Copy + 'static,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView
where
    F: Fn(&str, bool, MinMax<N>, Lng) -> Result<Option<N>, NumberEvalError<N>> + Copy + 'static,
    N: Copy + Clone + PartialEq + fmt::Display + 'static,
{
    let error = RwSignal::new(Option::<String>::None);
    let txt = RwSignal::new(String::new());
    let is_focussed = RwSignal::new(false);

    Effect::new(move |_| {
        if !is_focussed.get() {
            let new_value = input_value.get().map(format_value).unwrap_or_default();
            txt.set(new_value);
        }
        match evaluate_input(&txt.get(), required, limits, lng) {
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
        let result = evaluate_input(&input, required, limits, lng);
        txt.set(input);
        match result {
            Err(err) => {
                error.set(Some(err.to_string()));
                if err == NumberEvalError::Empty {
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
        if let Some(value) = input_value.get() {
            let new_txt = format_plain_value(value);
            txt.set(new_txt);
        }
    };

    // FIXME: Format input as string and remove delemiters
    // FIXME: trigger focusout on Enter or Escape

    view! {
      <div>
        { move || create_tooltip(label.clone(), description, required, accessibility_always_show_option, id) }
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = id.to_string()
            type="text"
            class = "block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            class = ("bg-red-100", move || error.get().is_some())
            placeholder = placeholder
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
          <p class="mt-2 text-sm" style="color: red">
            { move || error.get() }
          </p>
        </Show>
      </div>
    }
}

type FloatEvalError = NumberEvalError<f64>;

// TODO: Map error messages with current language
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
    lng: Lng,
) -> Result<Option<f64>, FloatEvalError> {
    evaluate_number_input(txt, required, limits, |x| lng.parse_str_as_f64(x).ok())
}

fn evaluate_u64_input(
    txt: &str,
    required: bool,
    limits: MinMax<u64>,
    lng: Lng,
) -> Result<Option<u64>, NumberEvalError<u64>> {
    evaluate_number_input(txt, required, limits, |x| {
        lng.parse_str_as_f64(x).ok().map(|v| v as u64)
    })
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
fn BoolInput(
    label: Signal<String>,
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
