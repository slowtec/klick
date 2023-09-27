use leptos::*;

#[derive(Debug)]
pub enum Field<ID> {
    Float {
        base: FieldBase<ID>,
        initial_value: Option<f64>,
        placeholder: Option<&'static str>,
        min_value: Option<f64>,
        max_value: Option<f64>,
        unit: &'static str,
    },
    Text {
        base: FieldBase<ID>,
        initial_value: Option<String>,
        placeholder: Option<&'static str>,
        max_len: Option<usize>,
    },
    Bool {
        base: FieldBase<ID>,
        initial_value: Option<bool>,
    },
    Selection {
        base: FieldBase<ID>,
        initial_value: Option<i32>,
        options: Vec<SelectOption>,
    },
}

impl<ID: Copy> Field<ID> {
    const fn id(&self) -> ID {
        match self {
            Self::Float { base, .. } => base.id,
            Self::Text { base, .. } => base.id,
            Self::Bool { base, .. } => base.id,
            Self::Selection { base, .. } => base.id,
        }
    }
}

#[derive(Debug)]
pub struct FieldBase<ID> {
    pub id: ID,
    pub label: &'static str,
    pub description: Option<&'static str>,
    pub required: bool,
}

#[derive(Debug)]
pub struct FieldSet<ID> {
    pub title: &'static str,
    pub fields: Vec<Field<ID>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SelectOption {
    pub label: &'static str,
    pub value: i32,
}

pub enum FieldSignal {
    Float((ReadSignal<Option<f64>>, WriteSignal<Option<f64>>)),
    Text((ReadSignal<Option<String>>, WriteSignal<Option<String>>)),
    Bool((ReadSignal<Option<bool>>, WriteSignal<Option<bool>>)),
    Selection((ReadSignal<Option<i32>>, WriteSignal<Option<i32>>)),
}

pub fn render_field_sets<ID>(
    field_sets: Vec<FieldSet<ID>>,
) -> (Vec<(ID, FieldSignal)>, Vec<impl IntoView>)
where
    ID: Into<&'static str> + Copy,
{
    let mut signals = vec![];
    let mut set_views = vec![];

    for set in field_sets {
        let mut field_views = vec![];

        for field in set.fields {
            let id = field.id();
            let (field_signal, view) = render_field(field);
            field_views.push(view);
            signals.push((id, field_signal));
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
    ID: Into<&'static str> + Copy,
{
    match field {
        Field::Text {
            base,
            placeholder,
            initial_value,
            max_len,
            ..
        } => {
            let signal = create_signal(initial_value);
            let field_signal = FieldSignal::Text(signal);
            let view = view! {
              <TextInput
                label = base.label
                name = base.id.into()
                placeholder = placeholder.unwrap()
                value = signal.0.get().unwrap_or_default()
                max_len
              />
            }
            .into_view();
            (field_signal, view)
        }
        Field::Float {
            base,
            placeholder,
            unit,
            initial_value,
            ..
        } => {
            let signal = create_signal(initial_value);
            let field_signal = FieldSignal::Float(signal);

            let view = view! {
              <NumberInput
                label = base.label
                name = base.id.into()
                placeholder = placeholder.unwrap()
                value = signal.0.get().unwrap_or_default().to_string().replace('.',",")
                unit
              />
            }
            .into_view();
            (field_signal, view)
        }
        Field::Bool {
            base,
            initial_value,
            ..
        } => {
            let signal = create_signal(initial_value);
            let field_signal = FieldSignal::Bool(signal);
            let view = view! {
              <BoolInput
                label = base.label
                name = base.id.into()
                value = signal.0.get().unwrap_or_default()
                comment = base.description
              />
            }
            .into_view();
            (field_signal, view)
        }
        Field::Selection {
            base,
            initial_value,
            options,
        } => {
            let signal = create_signal(initial_value);
            let field_signal = FieldSignal::Selection(signal);
            let view = view! {
              <SelectInput
                label = base.label
                name = base.id.into()
                value = signal.0.get().unwrap_or_default()
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
    name: &'static str,
    placeholder: &'static str,
    value: String,
    max_len: Option<usize>,
) -> impl IntoView {
    let input_id = format!("form-input-{name}");

    view! {
      <div>
        <label for={ &input_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            type = "text"
            id = { input_id }
            name = { name }
            maxlength = { max_len }
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            value = { value }
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
    name: &'static str,
    value: String,
) -> impl IntoView {
    let input_id = format!("form-number-input-{name}");

    view! {
      <div>
        <label for={ &input_id } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <div class="relative mt-2 rounded-md shadow-sm">
          <input
            id = { input_id }
            type="text"
            name = { name }
            class="block w-full rounded-md border-0 py-1.5 pr-12 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
            placeholder= { placeholder }
            // TODO: aria-describedby
            value = { value }
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
    name: &'static str,
    value: bool,
    comment: Option<&'static str>,
) -> impl IntoView {
    let input_id = format!("form-bool-input-{name}");

    view! {
      <div class="relative flex items-start">
        <div class="flex h-6 items-center">
          <input
            id  = { &input_id }
            name = { name }
            type="checkbox"
            class="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
            // TODO: aria-describedby
            checked = value
          />
        </div>
        <div class="ml-3 text-sm leading-6">
          <label for={ input_id } class="font-bold text-gray-900">{ label }</label>
          <p class="text-gray-500">{ comment }</p>
        </div>
      </div>
    }
}

#[component]
fn SelectInput(
    label: &'static str,
    name: &'static str,
    value: i32,
    options: Vec<SelectOption>,
) -> impl IntoView {
    let id = format!("form-select-input-{name}");

    view! {
      <div>
        <label for={ id.clone() } class="block text-sm font-bold leading-6 text-gray-900">{ label }</label>
        <select
          name = { name }
          id = { id }
          class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
        >
          <For
            each = move || options.clone()
            key = |option| option.value
            view = move |option| view! {
              <option value=option.value selected = (value == option.value) >{ option.label }</option>
            }
          />
        </select>
      </div>
    }
}
