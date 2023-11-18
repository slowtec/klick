use std::{collections::HashMap, rc::Rc};

use leptos::{ev::MouseEvent, *};
use strum::IntoEnumIterator;

use klick_application as app;
use klick_boundary::{
    AnnualAverages, EnergyConsumption, InputData, N2oEmissionFactorCalcMethod, OperatingMaterials,
    SewageSludgeTreatment,
};
use klick_svg_charts::BarChart;

use crate::{
    forms::{self, FieldSignal},
    sankey,
};

mod example_data;
mod fields;

use self::fields::FieldId;

const CHART_ELEMENT_ID: &str = "chart";

#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool() -> impl IntoView {
    let field_sets = fields::field_sets();

    let (signals, set_views) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);

    let input_data = RwSignal::new(Option::<app::InputData>::None);

    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let barchart_arguments: RwSignal<Vec<klick_svg_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let data = read_input_fields(&s);
        input_data.set(data);
    });

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        if let Some(input_data) = input_data.get() {
            let use_custom_factor = s
                .get(&FieldId::CustomN2oScenarioSupport)
                .and_then(FieldSignal::get_bool)
                == Some(true);
            if !use_custom_factor && selected_scenario.get() == Some(4) {
                selected_scenario.set(Some(0));
            }
            log::debug!("Calculating with {input_data:#?}");
            let szenario_calculations = N2oEmissionFactorCalcMethod::iter()
            .enumerate()
            .filter_map(|(i, method)| {

                  let calc_method = match method {
                      N2oEmissionFactorCalcMethod::CustomFactor => {
                          let custom_factor = s
                              .get(&FieldId::CustomN2oScenarioValue)
                              .and_then(FieldSignal::get_float).unwrap_or_default() / 100.0;
                          app::N2oEmissionFactorCalcMethod::CustomFactor(custom_factor)
                      }
                      N2oEmissionFactorCalcMethod::ExtrapolatedParravicini=>  app::N2oEmissionFactorCalcMethod::ExtrapolatedParravicini,
                      N2oEmissionFactorCalcMethod::Optimistic             =>  app::N2oEmissionFactorCalcMethod::Optimistic,
                      N2oEmissionFactorCalcMethod::Pesimistic             =>  app::N2oEmissionFactorCalcMethod::Pesimistic,
                      N2oEmissionFactorCalcMethod::Ipcc2019               =>  app::N2oEmissionFactorCalcMethod::Ipcc2019,
                  };

                 let output_data = klick_application::calc(&input_data, calc_method);

                 if selected_scenario.get() == Some(i as u64) {
                     let name_ka: String = s
                         .get(&FieldId::Name)
                         .and_then(FieldSignal::get_text)
                         .unwrap_or_else(|| "Kläranlage".to_string());

                     let ew = s
                         .get(&FieldId::Ew)
                         .and_then(FieldSignal::get_float)
                         .unwrap_or_default();

                     let einheit = "t CO₂-eq/Jahr";
                     let szenario_name = label_of_n2o_emission_factor_calc_method(&method);
                     let title = format!(
                         "{name_ka} ({ew} EW) / Treibhausgasemissionen [{einheit}] - Szenario {szenario_name}"
                     );
                     sankey_header.set(title);
                     sankey::render(output_data.clone(), CHART_ELEMENT_ID);
                 }
                 if matches!(method, N2oEmissionFactorCalcMethod::CustomFactor) && !use_custom_factor
                 {
                     None
                 } else {
                    Some((method, output_data))
                 }
             })
             .collect::<Vec<_>>();

            barchart_arguments.set(
                szenario_calculations
                    .iter()
                    .map(|(szenario, d)| klick_svg_charts::BarChartArguments {
                        label: Some(label_of_n2o_emission_factor_calc_method(szenario)),
                        co2_data: d.emissionen_co2_eq,
                        n2o_factor: d.ef_n2o_anlage,
                    })
                    .collect(),
            );
        } else {
            sankey_header.set(String::new());
            barchart_arguments.update(std::vec::Vec::clear);
            sankey::clear(CHART_ELEMENT_ID);
        }
    });

    view! {
      <div class="space-y-12">
        <div class="flex items-center justify-end gap-x-6">
          <Button
            label = "alle Werte löschen"
            on_click = {
              let signals = Rc::clone(&signals);
              move |_| {
                for s in signals.values() { s.clear(); }
              }
            }
          />
          <Button
            label = "Beispielwerte laden"
            on_click = {
              let signals = Rc::clone(&signals);
              move |_| {
                example_data::load_example_field_signal_values(&signals);
              }
            }
          />
        </div>
        { set_views }
      </div>
      // bar diagram
      { move ||
        {
          if barchart_arguments.get().is_empty() {
            None
          } else {
            Some(view! {
              <h3 class="my-8 text-xl font-bold">"Szenarien im Vergleich - Treibhausgasemissionen [t CO₂-eq/Jahr]"</h3>
              <div class="">
                <BarChart
                  width = 1200.0
                  height = 400.0
                  data  = barchart_arguments.into()
                  selected_bar = selected_scenario
                />
              </div>
            })
          }
        }
      }
      // sankey diagram
      <Show when= move || { sankey_header.get() != ""}>
      <h3 class="my-8 text-xl font-bold">
      { move ||
         sankey_header.get().to_string()
      }
      </h3>

      </Show>
      <div id={ CHART_ELEMENT_ID } class="mt-8"></div>
    }
}

const fn label_of_n2o_emission_factor_calc_method(
    method: &N2oEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => "Extrapoliert",
        N2oEmissionFactorCalcMethod::Optimistic => "Optimistisch",
        N2oEmissionFactorCalcMethod::Pesimistic => "Pessimistisch",
        N2oEmissionFactorCalcMethod::Ipcc2019 => "IPCC 2019",
        N2oEmissionFactorCalcMethod::CustomFactor => "Benutzerdefiniert",
    }
}

#[component]
fn Button<F>(label: &'static str, on_click: F) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    view! {
      <button
        type="button"
        on:click = on_click
        class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
        { label }
      </button>
    }
}

fn read_input_fields(s: &HashMap<FieldId, FieldSignal>) -> Option<app::InputData> {
    let plant_name = s.get(&FieldId::Name).and_then(FieldSignal::get_text);
    let population_values = s.get(&FieldId::Ew).and_then(FieldSignal::get_float);
    let waste_water = s.get(&FieldId::Flow).and_then(FieldSignal::get_float);

    let inflow_averages = AnnualAverages {
        nitrogen: s.get(&FieldId::TknZu).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbZu).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PZu).and_then(FieldSignal::get_float),
    };
    let effluent_averages = AnnualAverages {
        nitrogen: s.get(&FieldId::TknAb).and_then(FieldSignal::get_float),
        chemical_oxygen_demand: s.get(&FieldId::CsbAb).and_then(FieldSignal::get_float),
        phosphorus: s.get(&FieldId::PAb).and_then(FieldSignal::get_float),
    };

    let energy_consumption = EnergyConsumption {
        sewage_gas_produced: s.get(&FieldId::Klaergas).and_then(FieldSignal::get_float),
        methane_level: s
            .get(&FieldId::Methangehalt)
            .and_then(FieldSignal::get_float),
        gas_supply: s.get(&FieldId::GasZusatz).and_then(FieldSignal::get_float),
        purchase_of_biogas: s.get(&FieldId::Biogas).and_then(FieldSignal::get_bool),
        total_power_consumption: s
            .get(&FieldId::Strombedarf)
            .and_then(FieldSignal::get_float),
        in_house_power_generation: s.get(&FieldId::Eigenstrom).and_then(FieldSignal::get_float),
        emission_factor_electricity_mix: s
            .get(&FieldId::EfStrommix)
            .and_then(FieldSignal::get_float),
    };

    let sewage_sludge_treatment = SewageSludgeTreatment {
        open_sludge_bags: s
            .get(&FieldId::Schlammtaschen)
            .and_then(FieldSignal::get_bool),
        open_sludge_storage_containers: s
            .get(&FieldId::Schlammstapel)
            .and_then(FieldSignal::get_bool),
        sewage_sludge_for_disposal: s
            .get(&FieldId::KlaerschlammEnstorgung)
            .and_then(FieldSignal::get_float),
        transport_distance: s
            .get(&FieldId::KlaerschlammTransport)
            .and_then(FieldSignal::get_float),
    };

    let operating_materials = OperatingMaterials {
        fecl3: s
            .get(&FieldId::BetriebsstoffeFe3)
            .and_then(FieldSignal::get_float),
        feclso4: s
            .get(&FieldId::BetriebsstoffeFeso4)
            .and_then(FieldSignal::get_float),
        caoh2: s
            .get(&FieldId::BetriebsstoffeKalk)
            .and_then(FieldSignal::get_float),
        synthetic_polymers: s
            .get(&FieldId::BetriebsstoffePoly)
            .and_then(FieldSignal::get_float),
    };

    let input_data = InputData {
        plant_name,
        population_values,
        waste_water,
        inflow_averages,
        effluent_averages,
        energy_consumption,
        sewage_sludge_treatment,
        operating_materials,
    };

    app::InputData::try_from(input_data).ok()
}
