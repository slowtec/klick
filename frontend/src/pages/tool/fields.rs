use leptos::*;

use num_traits::ToPrimitive;
use strum::IntoEnumIterator;

use klick_app_components::forms::{self, *};
use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id, Value, ValueType};
use klick_presenter::{
    metadata_of, FieldMetaData, InputValueFieldType, InputValueFieldTypeHint, Lng, Placeholder,
};

use crate::{current_lang, label_signal};

fn form_limits(id: &Id) -> MinMax<f64> {
    MinMax {
        min: id.min(),
        max: id.max(),
    }
}

pub fn create_field(write: WriteSignal<FormData>, read: Signal<FormData>, id: Id) -> Field {
    let meta = metadata_of(&id);
    let lang = current_lang();

    let placeholder: Option<Signal<String>> = match meta.placeholder {
        Placeholder::Text(key) => Some(Signal::derive(move || {
            FieldMetaData::lookup(lang.get(), key)
        })),
        Placeholder::DefaultValue => format_default_value(id),
        Placeholder::Label => Some(label_signal(id)),
        Placeholder::None => None,
    };
    let field_type = create_field_type(write, read, id, placeholder);

    // FIXME: use signal here
    let description = Some(FieldMetaData::lookup(lang.get(), meta.description));

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
        let lang = current_lang().get();
        let default_text = match lang {
            Lng::De => "Standardwert",
            Lng::En => "default value",
        };
        format!("{} ({})", lang.format_value(&default_value), default_text)
    }))
}

pub fn create_field_type(
    write: WriteSignal<FormData>,
    read: Signal<FormData>,
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
        ValueType::Enum(enum_kind) => {
            let on_change = match enum_kind {
                EnumType::N2oEmissionFactorCalcMethod => Callback::new(move |_: Option<usize>| {}),
                EnumType::Ch4ChpEmissionFactorCalcMethod => {
                    Callback::new(move |_: Option<usize>| {})
                }
            };

            let input = match enum_kind {
                EnumType::N2oEmissionFactorCalcMethod => Signal::derive(move || {
                    read.with(|d| {
                        d.get(&id)
                            .cloned()
                            .map(Value::as_n2o_emission_factor_calc_method_unchecked)
                            .and_then(|v| v.to_usize())
                    })
                }),
                EnumType::Ch4ChpEmissionFactorCalcMethod => Signal::derive(move || {
                    read.with(|d| {
                        d.get(&id)
                            .cloned()
                            .map(Value::as_ch4_chp_emission_factor_calc_method_unchecked)
                            .and_then(|v| v.to_usize())
                    })
                }),
            };

            let options = match enum_kind {
                EnumType::N2oEmissionFactorCalcMethod => N2oEmissionFactorCalcMethod::iter()
                    .map(|c| format!("{c:?}"))
                    .collect(),

                EnumType::Ch4ChpEmissionFactorCalcMethod => Ch4ChpEmissionFactorCalcMethod::iter()
                    .map(|c| format!("{c:?}"))
                    .collect(),
            };

            FieldType::DropDown {
                options,
                initial_value: None, // TODO
                on_change,
                input,
            }
        }
    }
}
