use leptos::*;

use klick_app_components::forms::MinMax;
use klick_domain as domain;
use klick_presenter::Lng;

use super::Card;

use crate::pages::tool::FieldSignal;
use crate::{
    forms::{render_field_sets, FieldType},
    pages::tool::{
        field_sets::{Field, FieldSet},
        fields::ScenarioFieldId,
        FieldId,
    },
};

use klick_domain::units::Factor;

pub fn options(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    process_energy_savings: RwSignal<Option<f64>>,
    fossil_energy_savings: RwSignal<Option<f64>>,
    district_heating: RwSignal<Option<f64>>,
    photovoltaic_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_photovoltaic_usage: RwSignal<Option<f64>>,
    wind_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_wind_energy_usage: RwSignal<Option<f64>>,
    water_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_water_energy_usage: RwSignal<Option<f64>>,
) -> impl IntoView {
    let field_sets = field_sets();
    let (signals, view, _required_fields) = render_field_sets(field_sets);

    let excess_energy_co2_equivalent = RwSignal::new(0.0);
    let emissions = RwSignal::new(0.0);
    let electricity_mix = RwSignal::new(0.0);
    let electricity_mix_savings = RwSignal::new(0.0);

    create_effect(move |_| match output.get() {
        None => {
            excess_energy_co2_equivalent.set(0.0);
            emissions.set(0.0);
            electricity_mix.set(0.0);
            electricity_mix_savings.set(0.0);
        }
        Some(domain::EmissionsCalculationOutcome {
            co2_equivalents, ..
        }) => {
            excess_energy_co2_equivalent.set(co2_equivalents.excess_energy_co2_equivalent.into());
            emissions.set(co2_equivalents.total_emissions.into());
            electricity_mix.set(co2_equivalents.electricity_mix.into());
            electricity_mix_savings.set(
                ((co2_equivalents.total_emissions - co2_equivalents.excess_energy_co2_equivalent)
                    * Factor::new(-1.0))
                .into(),
            );
        }
    });

    create_effect(move |_| {
        let process_energy_savings_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::ProcessEnergySaving).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(process_energy_savings_) = process_energy_savings_ {
            process_energy_savings.set(process_energy_savings_.get());
        }

        let fossil_energy_savings_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::FossilEnergySaving).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(fossil_energy_savings_) = fossil_energy_savings_ {
            fossil_energy_savings.set(fossil_energy_savings_.get());
        }
        let district_heating_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::DistrictHeating).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(district_heating_) = district_heating_ {
            district_heating.set(district_heating_.get());
        }

        let photovoltaic_energy_expansion_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::PhotovoltaicEnergyExpansion).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(photovoltaic_energy_expansion_) = photovoltaic_energy_expansion_ {
            photovoltaic_energy_expansion.set(photovoltaic_energy_expansion_.get());
        }
        let estimated_self_photovoltaic_usage_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::EstimatedSelfPhotovolaticUsage).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(estimated_self_photovoltaic_usage_) = estimated_self_photovoltaic_usage_ {
            estimated_self_photovoltaic_usage.set(estimated_self_photovoltaic_usage_.get());
        }

        let wind_energy_expansion_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::WindEnergyExpansion).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(wind_energy_expansion_) = wind_energy_expansion_ {
            wind_energy_expansion.set(wind_energy_expansion_.get());
        }
        let estimated_self_wind_energy_usage_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::EstimatedSelfWindEnergyUsage).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(estimated_self_wind_energy_usage_) = estimated_self_wind_energy_usage_ {
            estimated_self_wind_energy_usage.set(estimated_self_wind_energy_usage_.get());
        }

        let water_energy_expansion_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::WaterEnergyExpansion).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(water_energy_expansion_) = water_energy_expansion_ {
            water_energy_expansion.set(water_energy_expansion_.get());
        }
        let estimated_self_water_energy_usage_ = signals
            .get(&FieldId::Scenario(ScenarioFieldId::EstimatedSelfWaterEnergyUsage).into())
            .and_then(FieldSignal::get_float_output_signal);
        if let Some(estimated_self_water_energy_usage_) = estimated_self_water_energy_usage_ {
            estimated_self_water_energy_usage.set(estimated_self_water_energy_usage_.get());
        }
    });

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
            when= move || (excess_energy_co2_equivalent.get() > 0.0)
          >
            " Ihre Kläranlage ist energieneutral. Die Kläranlage spart "
            { Lng::De.format_number_with_thousands_seperator(electricity_mix_savings.get()) }
            " t CO2-Äq./a ein."
          </Show>
          <Show
            when= move || (excess_energy_co2_equivalent.get() <= 0.0)
          >
            "Ihre Kläranlage benötigt weiterhin externen Strom (Versorger), wodurch "
            { Lng::De.format_number_with_thousands_seperator(electricity_mix.get()) }
            " t CO₂-Äq./a energiebedingte Emissionen entstehen."
          </Show>
        </p>
        <div class="border-t pt-3 mt-4 border-gray-900/10">
          { move || {
              output.get().map(|out|
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
              )
            }
          }
        </div>
      </Card>
    }
}

fn field_sets() -> Vec<FieldSet> {
    vec![
        FieldSet {
            title: Some("Prozesse und fossile Energieträger"),
            fields: vec![
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::ProcessEnergySaving).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahreseinsparung".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                    },
                },
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::FossilEnergySaving).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahreseinsparung".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Photovoltaik"),
            fields: vec![
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::PhotovoltaicEnergyExpansion).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh",
                    },
                },
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::EstimatedSelfPhotovolaticUsage).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Windkraft"),
            fields: vec![
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::WindEnergyExpansion).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh",
                    },
                },
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::EstimatedSelfWindEnergyUsage).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("".to_string().to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Wasserkraft"),
            fields: vec![
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::WaterEnergyExpansion).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("Jahresleistung".to_string()),
                        limits: MinMax {
                            min: None,
                            max: None,
                        },
                        unit: "kWh",
                    },
                },
                Field {
                    id: FieldId::Scenario(ScenarioFieldId::EstimatedSelfWaterEnergyUsage).into(),
                    description: Some(""),
                    required: false,
                    field_type: FieldType::Float {
                        initial_value: None,
                        placeholder: Some("".to_string()),
                        limits: MinMax {
                            min: Some(0.0),
                            max: Some(100.0),
                        },
                        unit: "%",
                    },
                },
            ],
        },
        FieldSet {
            title: Some("Abwärmenutzung"),
            fields: vec![Field {
                id: FieldId::Scenario(ScenarioFieldId::DistrictHeating).into(),
                description: Some(""),
                required: false,
                field_type: FieldType::Float {
                    initial_value: None,
                    placeholder: Some("Jahresleistung".to_string()),
                    limits: MinMax {
                        min: None,
                        max: None,
                    },
                    unit: "kWh",
                },
            }],
        },
    ]
}
