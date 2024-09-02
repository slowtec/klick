use leptos::*;

use klick_app_components::forms::{self, *};
use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id, Value, ValueType};
use klick_presenter::{metadata, InputValueFieldType, InputValueFieldTypeHint, Placeholder};

use crate::{current_lang, label_signal};

fn form_limits(id: &Id) -> MinMax<f64> {
    MinMax {
        min: id.min(),
        max: id.max(),
    }
}

pub fn create_field(write: WriteSignal<FormData>, read: ReadSignal<FormData>, id: Id) -> Field {
    let meta = metadata(&id);
    let placeholder: Option<Signal<String>> = match meta.placeholder {
        Placeholder::Text(txt) => Some(RwSignal::new(txt.to_string()).into()),
        Placeholder::DefaultValue => format_default_value(id),
        Placeholder::Label => Some(label_signal(id)),
        Placeholder::None => None,
    };
    let field_type = create_field_type(write, read, id, placeholder);
    let description = Some(meta.description);
    Field {
        label: label_signal(id),
        description,
        required: !id.is_optional(), // TODO: check for default value if not optional
        field_type,
    }
}

fn format_default_value(id: Id) -> Option<Signal<String>> {
    let default_value = id.default_value()?;
    Some(Signal::derive(move || {
        let lng = current_lang().get();
        lng.format_value(&default_value)
    }))
}

pub fn create_field_type(
    write: WriteSignal<FormData>,
    read: ReadSignal<FormData>,
    id: Id,
    placeholder: Option<Signal<String>>,
) -> FieldType {
    let limits = form_limits(&id);
    let value_type = id.value_type();

    match value_type {
        ValueType::Scalar(scalar) => match scalar {
            ScalarType::Float(float_type) => {
                let initial_value = None; // TODO
                let on_change = Callback::new(move |v: Option<_>| {
                    let value = v
                        .map(|v| Float::from_f64_with_type(v, float_type))
                        .map(Scalar::Float)
                        .map(Value::Scalar);
                    write.update(|d| {
                        if let Some(value) = value {
                            d.insert(id, value);
                        } else {
                            d.remove(&id);
                        }
                    });
                });

                let input = Signal::derive(move || {
                    read.with(|d| {
                        d.get(&id)
                            .cloned()
                            .map(Value::as_float_unchecked)
                            .map(f64::from)
                    })
                });

                let unit = float_type.abbreviation();
                FieldType::Float {
                    initial_value,
                    placeholder,
                    limits,
                    unit,
                    on_change,
                    input,
                }
            }
            ScalarType::Bool => {
                let on_change = Callback::new(move |v| {
                    write.update(|d| {
                        d.insert(id, Value::bool(v));
                    });
                });
                let input = Signal::derive(move || {
                    read.with(|d| d.get(&id).cloned().is_some_and(Value::as_bool_unchecked))
                });
                FieldType::Bool {
                    initial_value: None, // TODO
                    on_change,
                    input,
                }
            }
            ScalarType::Int(IntType::Count) => {
                let on_change = Callback::new(move |v: Option<_>| {
                    write.update(|d| {
                        if let Some(value) = v.map(Value::count) {
                            d.insert(id, value);
                        } else {
                            d.remove(&id);
                        }
                    });
                });
                let input = Signal::derive(move || {
                    read.with(|d| {
                        d.get(&id)
                            .cloned()
                            .map(Value::as_count_unchecked)
                            .map(u64::from)
                    })
                });
                let limits = forms::MinMax {
                    min: limits.min.map(|min| min as u64),
                    max: limits.max.map(|max| max as u64),
                };
                FieldType::UnsignedInteger {
                    initial_value: None, // TODO
                    placeholder,
                    unit: "", // TODO
                    limits,
                    on_change,
                    input,
                }
            }
        },
        ValueType::Text => {
            let on_change = Callback::new(move |v: Option<_>| {
                write.update(|d| {
                    if let Some(value) = v.map(Value::text) {
                        d.insert(id, value);
                    } else {
                        d.remove(&id);
                    }
                });
            });
            let input = Signal::derive(move || {
                read.with(|d| d.get(&id).cloned().map(Value::as_text_unchecked))
            });
            let max_len = limits.max.map(|max| max as usize);

            match id.field_type_hint() {
                Some(InputValueFieldType::TextArea) => {
                    todo!()
                }
                None | Some(InputValueFieldType::TextInput) => {
                    FieldType::Text {
                        initial_value: None, // TODO
                        placeholder,
                        max_len,
                        on_change,
                        input,
                    }
                }
                hint => panic!("Invalid field type hint {hint:?} for text value"),
            }
        }
        ValueType::Enum(_) => todo!(),
    }
}
