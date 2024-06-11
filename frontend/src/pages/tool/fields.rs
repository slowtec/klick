use leptos::*;

use klick_app_components::forms::{self, *};
use klick_boundary::FormData;
use klick_domain::{units::*, InputValueId as Id, Value, ValueType};

pub enum Limits {
    Float(forms::MinMax<f64>),
    Uint(forms::MinMax<u64>),
}

pub fn create_field_type(
    data: RwSignal<FormData>,
    id: Id,
    value_type: ValueType,
    placeholder: Option<String>,
    limits: Option<Limits>,
) -> FieldType {
    match value_type {
        ValueType::Scalar(ScalarType::Float(float_type)) => {
            let initial_value = None; // TODO
            let limits = match limits {
                Some(Limits::Float(limits)) => limits,
                None => forms::MinMax {
                    min: None,
                    max: None,
                },
                _ => panic!("expected float limits for {value_type:?}"),
            };
            let (on_change, input, unit) = match float_type {
                FloatType::Density(DensityType::MilligramsPerLiter) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        data.update(|d| d.set(id, v.map(Value::milligrams_per_liter)));
                    });
                    let input = Signal::derive(move || {
                        data.with(|d| {
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
                        data.update(|d| d.set(id, v.map(Value::qubicmeters)));
                    });
                    let input = Signal::derive(move || {
                        data.with(|d| {
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
                        data.update(|d| d.set(id, v.map(Value::tons)));
                    });
                    let input = Signal::derive(move || {
                        data.with(|d| d.get(&id).map(Value::as_tons_unchecked).map(f64::from))
                    });
                    let unit = "t";
                    (on_change, input, unit)
                }
                FloatType::Energy(EnergyType::Kilowatthours) => {
                    let on_change = Callback::new(move |v: Option<_>| {
                        data.update(|d| d.set(id, v.map(Value::kilowatthours)));
                    });
                    let input = Signal::derive(move || {
                        data.with(|d| {
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
                        data.update(|d| d.set(id, v.map(Value::grams_per_kilowatthour)));
                    });
                    let input = Signal::derive(move || {
                        data.with(|d| {
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
                data.update(|d| d.set(id, Some(Value::bool(v))));
            });
            let input = Signal::derive(move || {
                data.with(|d| d.get(&id).map(Value::as_bool_unchecked).unwrap_or_default())
            });
            FieldType::Bool {
                initial_value: None, // TODO
                on_change,
                input,
            }
        }
        ValueType::Scalar(ScalarType::Int(IntType::Count)) => {
            let on_change = Callback::new(move |v| {
                data.update(|d| d.plant_profile.sewage_sludge_treatment.digester_count = v);
            });
            let input = Signal::derive(move || {
                data.with(|d| d.plant_profile.sewage_sludge_treatment.digester_count)
            });

            let limits = match limits {
                Some(Limits::Uint(limits)) => limits,
                None => forms::MinMax {
                    min: None,
                    max: None,
                },
                _ => panic!("expected integer limits for {value_type:?}"),
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
                data.update(|d| d.set(id, v.map(Value::text)));
            });
            let input =
                Signal::derive(move || data.with(|d| d.get(&id).map(Value::as_text_unchecked)));
            let max_len = match limits {
                Some(Limits::Uint(limits)) => limits.max.map(|max| max as usize),
                None => None,
                _ => panic!("expected float limits for {value_type:?}"),
            };
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
