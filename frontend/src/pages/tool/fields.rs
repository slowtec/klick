use leptos::*;

use klick_app_components::forms::{self, *};
use klick_boundary::FormData;
use klick_domain::{units::*, value_spec, InputValueId as Id, Value, ValueSpec, ValueType};
use klick_presenter::{metadata, Placeholder, ValueLabel};

fn form_limits(spec: &ValueSpec) -> MinMax<f64> {
    MinMax {
        min: spec.min(),
        max: spec.max(),
    }
}

pub fn create_field(write: WriteSignal<FormData>, read: ReadSignal<FormData>, id: Id) -> Field {
    let spec = value_spec(&id);
    let meta = metadata(&id);
    let placeholder = match meta.placeholder {
        Placeholder::Text(txt) => Some(txt.to_string()),
        Placeholder::DefaultValue => spec.default_value().map(|v| format!("{v:?}")), // FIXME: format value
        Placeholder::Label => Some(id.label().to_string()),
        Placeholder::None => None,
    };
    let field_type = create_field_type(write, read, id, placeholder);
    let description = Some(meta.description);
    Field {
        label: id.label(),
        description,
        required: !spec.optional(), // TODO: check for default value if not optional
        field_type,
    }
}

pub fn create_field_type(
    write: WriteSignal<FormData>,
    read: ReadSignal<FormData>,
    id: Id,
    placeholder: Option<String>,
) -> FieldType {
    let spec = value_spec(&id);
    let limits = form_limits(&spec);
    let value_type = spec.value_type();
    match value_type {
        ValueType::Scalar(ScalarType::Float(float_type)) => {
            let initial_value = None; // TODO
            let (on_change, input, unit) = match float_type {
                FloatType::Density(DensityType::MilligramsPerLiter) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        write.update(|d| d.set(id, v.map(Value::milligrams_per_liter)));
                    });
                    let input = Signal::derive(move || {
                        read.with(|d| {
                            d.get(&id)
                                .map(Value::as_milligrams_per_liter_unchecked)
                                .map(f64::from)
                        })
                    });
                    let unit = "mg/L"; // TODO: derive from value_type
                    (on_change, input, unit)
                }
                FloatType::Volume(VolumeType::Qubicmeters) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        write.update(|d| d.set(id, v.map(Value::qubicmeters)));
                    });
                    let input = Signal::derive(move || {
                        read.with(|d| {
                            d.get(&id)
                                .map(Value::as_qubicmeters_unchecked)
                                .map(f64::from)
                        })
                    });
                    let unit = "m³/a";
                    (on_change, input, unit)
                }
                FloatType::Mass(MassType::Tons) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        write.update(|d| d.set(id, v.map(Value::tons)));
                    });
                    let input = Signal::derive(move || {
                        read.with(|d| d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
                    });
                    let unit = "t";
                    (on_change, input, unit)
                }
                FloatType::Energy(EnergyType::Kilowatthours) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        write.update(|d| d.set(id, v.map(Value::kilowatthours)));
                    });
                    let input = Signal::derive(move || {
                        read.with(|d| {
                            d.get(&id)
                                .map(Value::as_kilowatthours_unchecked)
                                .map(f64::from)
                        })
                    });
                    let unit = "kWh/a";
                    (on_change, input, unit)
                }
                FloatType::SpecificEnergyDensity(
                    SpecificEnergyDensityType::GramsPerKilowatthour,
                ) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        write.update(|d| d.set(id, v.map(Value::grams_per_kilowatthour)));
                    });
                    let input = Signal::derive(move || {
                        read.with(|d| {
                            d.get(&id)
                                .map(Value::as_grams_per_kilowatthour_unchecked)
                                .map(f64::from)
                        })
                    });
                    let unit = "g CO₂/kWh";
                    (on_change, input, unit)
                }
                _ => todo!(),
            };
            FieldType::Float {
                initial_value,
                placeholder,
                limits,
                unit,
                on_change,
                input,
            }
        }
        ValueType::Scalar(ScalarType::Bool) => {
            let on_change = Callback::new(move |v| {
                write.update(|d| d.set(id, Some(Value::bool(v))));
            });
            let input = Signal::derive(move || {
                read.with(|d| d.get(&id).map(Value::as_bool_unchecked).unwrap_or_default())
            });
            FieldType::Bool {
                initial_value: None, // TODO
                on_change,
                input,
            }
        }
        ValueType::Scalar(ScalarType::Int(IntType::Count)) => {
            let on_change = Callback::new(move |v: Option<_>| {
                write.update(|d| d.set(id, v.map(Value::count)));
            });
            let input = Signal::derive(move || {
                read.with(|d| d.get(&id).map(Value::as_count_unchecked).map(u64::from))
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
        ValueType::Text => {
            let on_change = Callback::new(move |v: Option<_>| {
                write.update(|d| d.set(id, v.map(Value::text)));
            });
            let input =
                Signal::derive(move || read.with(|d| d.get(&id).map(Value::as_text_unchecked)));
            let max_len = limits.max.map(|max| max as usize);
            FieldType::Text {
                initial_value: None, // TODO
                placeholder,
                max_len,
                on_change,
                input,
            }
        }
        _ => todo!(),
    }
}
