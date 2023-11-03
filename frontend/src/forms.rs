use std::{
    collections::HashMap,
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

use inflector::cases::kebabcase::to_kebab_case;
use leptos::*;
use log::info;

pub use klick_boundary::{MinMax, Field, FieldSet, FieldType, SelectOption};

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
                placeholder = placeholder.unwrap()
                value = signal
                max_len
                description
              />
            }
            .into_view();
            (field_signal, view)
        }
        FieldType::Float {
            placeholder,
            unit,
            initial_value,
            plausible,
            unreasonable,
            ..
        } => {
            let signal = create_rw_signal(initial_value);
            let field_signal = FieldSignal::Float(signal);

            let view = view! {
              <NumberInput
                label
                field_id
                placeholder = placeholder.unwrap_or("")
                value = signal
                unit
                description
                plausible
                unreasonable
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
                //description
              />
            }
            .into_view();
            (field_signal, view)
        }
    }
}

fn create_tooltip(label: &'static str, description: Option<&'static str>, unit: Option<&'static str>, _plausible: Option<MinMax>, unreasonable: Option<MinMax>) -> impl IntoView {
    let show_tooltip: RwSignal<String> = create_rw_signal("none".to_string());
    let unreasonable_min = match unreasonable {
      Some(u) => u.min,
      None => None,
    };
    let unreasonable_max = match unreasonable {
        Some(u) => u.max,
        None => None,
    };

    view! {
        <div class="flex-col md:flex-row flex items-center md:justify-center">
          <a tabindex="0" role="link" aria-label="tooltip 1" class="focus:outline-none focus:ring-gray-300 rounded-full focus:ring-offset-2 focus:ring-2 focus:bg-gray-200 relative mt-20 md:mt-0"
          on:focus = move |_| {
            //info!("focus");
            show_tooltip.set("block".to_string());
          }
          on:blur = move |_| {
            //info!("blur");
            show_tooltip.set("none".to_string());
          }
          >
              <div class="cursor-pointer">
                  <svg aria-haspopup="true" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle" width="20" height="20" viewBox="0 0 24 24" stroke-width="1.5" stroke="#A0AEC0" fill="none" stroke-linecap="round" stroke-linejoin="round">
                      <path stroke="none" d="M0 0h24v24H0z" />
                      <circle cx="12" cy="12" r="9" />
                      <line x1="12" y1="8" x2="12.01" y2="8" />
                      <polyline points="11 12 12 12 12 16 13 16" />
                  </svg>
              </div>
              <div id="tooltip1" role="tooltip" class="z-20 -mt-20 w-64 absolute transition duration-150 ease-in-out left-0 ml-8 shadow-lg bg-white p-4 rounded" style={move || format!("display: {}", show_tooltip.get())}>
                  <svg class="absolute left-0 -ml-2 bottom-0 top-0 h-full" width="9px" height="16px" viewBox="0 0 9 16" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
                      <g id="Page-1" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                          <g id="Tooltips-" transform="translate(-874.000000, -1029.000000)" fill="#FFFFFF">
                              <g id="Group-3-Copy-16" transform="translate(850.000000, 975.000000)">
                                  <g id="Group-2" transform="translate(24.000000, 0.000000)">
                                      <polygon id="Triangle" transform="translate(4.500000, 62.000000) rotate(-90.000000) translate(-4.500000, -62.000000) " points="4.5 57.5 12.5 66.5 -3.5 66.5"></polygon>
                                  </g>
                              </g>
                          </g>
                      </g>
                  </svg>
                  <p class="text-sm font-bold text-gray-800 pb-1">{ label }</p>
                  <p class="text-xs leading-4 text-gray-600 pb-3">{ description }</p>
                  <Show when=move || (unreasonable_min.is_some() || unreasonable_max.is_some())>
                    //<p class="text-sm font-bold text-gray-800 pb-1">Plausiebel</p>
                    //<p class="block text-sm leading-6 text-gray-600">{ plausible.min } "< X " { unit } " < " {plausible.max} </p>
                    <p class="text-sm font-bold text-gray-800 pb-1">Grenzwerte Warnung</p>
                    <ul class="list-disc list-inside">
                    <Show when=move || unreasonable_min.is_some()>
                      <li class="text-xs leading-4 text-gray-600 pb-3">"Eingabe kleiner "  { unreasonable_min } " " { unit }</li>
                    </Show>
                    <Show when=move || unreasonable_max.is_some()>
                      <li class="text-xs leading-4 text-gray-600 pb-3">"Eingabe größer " { unreasonable_max } " " { unit }</li>
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
) -> impl IntoView {
    let unit: &'static str = "";
    view! {
      <div>
      <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
        <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        {create_tooltip(label, description, None, None, None)}
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
    description: Option<&'static str>,
    plausible: MinMax,
    unreasonable: MinMax,
) -> impl IntoView {
    view! {
      <div>
        <div class="block columns-2 sm:flex sm:justify-start sm:space-x-2">
          <label for={ &field_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
          {create_tooltip(label, description, Some(unit), Some(plausible), Some(unreasonable))}
        </div>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { field_id }
            type="text"
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            prop:value = move || value.get().map(|v|v.to_string().replace('.',",")).unwrap_or_default()
            on:input = move |ev| {
              info!("plausible {} {}", plausible.min.unwrap_or_default(), plausible.max.unwrap_or_default());
              info!("unreasonable {} {}", unreasonable.min.unwrap_or_default(), unreasonable.max.unwrap_or_default());
              let target_value = event_target_value(&ev);
              if target_value.is_empty() {
                value.set(None);
              } else if let Ok(target_value) = target_value.replace(',',".").parse() {
                match value.get() {
                  Some(v) => {
                    if target_value != v {
                      // fixes issue with signal-loop and incomplete numbers which
                      // get converted from "1," into "1" and prevent entering "," or "." separator
                      value.set(Some(target_value));
                    }
                  },
                  None => {
                    value.set(Some(target_value));
                  }
                }
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
    //description: Option<&'static str>,
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
