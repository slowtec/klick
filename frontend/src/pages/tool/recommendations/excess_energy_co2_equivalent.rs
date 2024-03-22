use leptos::*;

use klick_app_components::forms::*;
use klick_boundary::FormData;
use klick_domain::units::*;
use klick_presenter::*;

use crate::pages::tool::{CalculationOutcome, Card};

pub fn options(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    outcome: Signal<Option<CalculationOutcome>>,
) -> impl IntoView {
    // -----   ----- //
    //    Signals    //
    // -----   ----- //

    let excess_energy_co2_equivalent = Signal::derive(move || {
        outcome.with(|out| {
            out.as_ref().map(|out| {
                out.recommendation
                    .output
                    .co2_equivalents
                    .excess_energy_co2_equivalent
            })
        })
    });

    let electricity_mix_savings = Signal::derive(move || {
        outcome.with(|out| {
            out.as_ref().map(|out| {
                // TOOD: move this to calculation module
                let eq = &out.recommendation.output.co2_equivalents;
                (eq.total_emissions - eq.excess_energy_co2_equivalent) * Factor::new(-1.0)
            })
        })
    });

    let electricity_mix = Signal::derive(move || {
        outcome.with(|out| {
            out.as_ref()
                .map(|out| out.recommendation.output.co2_equivalents.electricity_mix)
        })
    });

    // -----   ----- //
    //    Fields     //
    // -----   ----- //

    let field_sets = field_sets(form_data.write_only(), input_data);
    let (view, _, _) = render_field_sets(field_sets);

    // -----   ----- //
    //     View      //
    // -----   ----- //

    view! {
      <Card title ="Energiebedingte Emissionen" bg_color="bg-yellow">
        <p>
        <b>"Energiesparmaßnahmen"</b>" und "<b>"Erneuerbare Energien"</b>" können maßgeblich zur Minderung indirekter Emissionen und
             zur Energieautarkie beitragen. Um die positiven Auswirkungen eines Zubaus der erneuerbaren Energien:
             Photovoltaik (PV), Wind-, Wasserkraft und/oder Abwärmenutzung aufzuzeigen, können nachfolgend verschiedene
             Szenarien bilanziert werden. Wenn Sie die jeweilige Technologie nicht bilanzieren wollen können Sie
             das jeweilige Feld freilassen setzen."
        </p>
        { view }
        <p>
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v > Some(Tons::zero()))
          >
            " Ihre Kläranlage ist energieneutral. Die Kläranlage spart "
            { electricity_mix_savings.with(|d|d.map(|v|Lng::De.format_number_with_thousands_seperator(f64::from(v)))) }
            " t CO2-Äq./a ein."
          </Show>
          <Show
            when= move || excess_energy_co2_equivalent.with(|v| *v <= Some(Tons::zero()))
          >
            "Ihre Kläranlage benötigt weiterhin externen Strom (Versorger), wodurch "
            { electricity_mix.with(|d|d.map(|v|Lng::De.format_number_with_thousands_seperator(f64::from(v)))) }
            " t CO₂-Äq./a energiebedingte Emissionen entstehen."
          </Show>
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
        { move || outcome.with(|out|out.as_ref().map(|out|{
            let out = out.recommendation.output.clone(); // TODO: avoid clone
            view! {
              <dl class="mx-3 my-2 grid grid-cols-2 text-sm">
                <Show when= move || (f64::from(out.co2_equivalents.process_energy_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Energieeinsparung bei Prozessen"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.process_energy_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.fossil_energy_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung bei Fossilen Energiequellen"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.fossil_energy_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.photovoltaic_expansion_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Photovoltaik"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.photovoltaic_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.wind_expansion_savings) > 0.0) >
                  <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Windkraft"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.wind_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.water_expansion_savings) > 0.0) >
                <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Wasserkraft"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.water_expansion_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
                <Show when= move || (f64::from(out.co2_equivalents.district_heating_savings) > 0.0) >
                <dt class="text-lg font-semibold text-right px-3 py-1 text-gray-500">"CO₂ Einsparung durch Abwärmenutzung"</dt>
                <dd class="text-lg py-1 px-3">
                  { format!("{:.1}", f64::from(out.co2_equivalents.district_heating_savings)).replace('.',",") }
                  <span class="ml-2 text-gray-400">{ "t CO₂-Äq./a" }</span>
                </dd>
                </Show>
              </dl>
            }
          }))
        }
        </div>
      </Card>
    }
}

fn field_sets(form_data: WriteSignal<FormData>, input_data: ReadSignal<FormData>) -> Vec<FieldSet> {
    vec![
        FieldSet {
            title: Some("Prozesse und fossile Energieträger"),
            fields: vec![
                Field {
                    label: ScenarioFieldId::ProcessEnergySaving.label(),
                    description: Some("Angabe der geschätzten Energieeinsparung bei Kläranlagen-Prozessen in Prozent (%)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahreseinsparung".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .process_energy_savings = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .process_energy_savings
                            })
                        }),
                    },
                },
                Field {
                    label: ScenarioFieldId::FossilEnergySaving.label(),
                    description: Some("Angabe der geschätzten Energieeinsparung bei fossilen Energieträgern (z.B. Heizöl/Erdgas) in Prozent (%)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahreseinsparung".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .fossil_energy_savings = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .fossil_energy_savings
                            })
                        }),
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Photovoltaik"),
            fields: vec![
                Field {
                    label: ScenarioFieldId::PhotovoltaicEnergyExpansion.label(),
                    description: Some("Angabe des Zubaus an Photovoltaikleistung in Kilowattstunden (kWh) pro Jahr (a)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh/a",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .photovoltaic_energy_expansion = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .photovoltaic_energy_expansion
                            })
                        }),
                    },
                },
                Field {
                    label: ScenarioFieldId::EstimatedSelfPhotovolaticUsage.label(),
                    description: Some("Geschätzte Eigennutzung der Photovoltaikleistung in Prozent (%)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some(String::new()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_photovoltaic_usage = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_photovoltaic_usage
                            })
                        }),
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Windkraft"),
            fields: vec![
                Field {
                    label: ScenarioFieldId::WindEnergyExpansion.label(),
                    description: Some("Angabe des Zubaus an Windkraftleistung in Kilowattstunden (kWh) pro Jahr (a)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh/a",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .wind_energy_expansion = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .wind_energy_expansion
                            })
                        }),
                    },
                },
                Field {
                    label: ScenarioFieldId::EstimatedSelfWindEnergyUsage.label(),
                    description: Some("Geschätzte Eigennutzung der Windkraftleistung in Prozent (%)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some(String::new().to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_wind_energy_usage = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_wind_energy_usage
                            })
                        }),
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Wasserkraft"),
            fields: vec![
                Field {
                    label: ScenarioFieldId::WaterEnergyExpansion.label(),
                    description: Some("Angabe des Zubaus an Wasserkraftleistung in Kilowattstunden (kWh) pro Jahr (a)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh/a",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .water_energy_expansion = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .water_energy_expansion
                            })
                        }),
                    },
                },
                Field {
                    label: ScenarioFieldId::EstimatedSelfWaterEnergyUsage.label(),
                    description: Some("Geschätzte Eigennutzung der Wasserkraftleistung in Prozent (%)."),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some(String::new()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                        on_change: Callback::new(move |v| {
                            form_data.update(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_water_energy_usage = v;
                            });
                        }),
                        input: Signal::derive(move || {
                            input_data.with(|d| {
                                d.optimization_scenario
                                    .energy_emissions
                                    .estimated_self_water_energy_usage
                            })
                        }),
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Abwärmenutzung"),
            fields: vec![Field {
                label: ScenarioFieldId::DistrictHeating.label(),
                description: Some("Angabe der Abgabeleistung an Fern-/Nahwärme in Kilowattstunden (kWh) pro Jahr (a)."),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some("Jahresleistung".to_string()),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "kWh/a",
                    on_change: Callback::new(move |v| {
                        form_data.update(|d| {
                            d.optimization_scenario.energy_emissions.district_heating = v;
                        });
                    }),
                    input: Signal::derive(move || {
                        input_data
                            .with(|d| d.optimization_scenario.energy_emissions.district_heating)
                    }),
                },
            }],
        },
    ]
}
