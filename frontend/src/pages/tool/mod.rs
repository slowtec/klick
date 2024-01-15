use std::rc::Rc;

use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;
use strum::IntoEnumIterator;

use klick_application as app;
use klick_boundary::{export_to_vec_pretty, import_from_slice, N2oEmissionFactorCalcMethod};
use klick_domain as domain;
use klick_format_numbers::Lng;
use klick_svg_charts::BarChart;

use crate::{
    forms::{self, FieldSignal, MissingField},
    sankey,
};

mod action_panel;
mod breadcrumbs;
mod example_data;
mod field_sets;
mod fields;
mod input_data_list;
mod optimization_options;

use self::{
    action_panel::ActionPanel,
    breadcrumbs::Breadcrumbs,
    field_sets::field_sets,
    fields::{load_fields, read_input_fields, read_scenario_fields, FieldId, ScenarioFieldId},
    input_data_list::InputDataList,
    optimization_options::OptimizationOptions,
};

const CHART_ELEMENT_ID: &str = "chart";

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PageSection {
    #[default]
    DataCollection,
    OptimizationOptions,
}

impl PageSection {
    const fn section_id(self) -> &'static str {
        match self {
            PageSection::DataCollection => "data-collection",
            PageSection::OptimizationOptions => "optimization-options",
        }
    }
}

#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool() -> impl IntoView {
    let field_sets = field_sets();
    let (signals, set_views, required_fields) = forms::render_field_sets(field_sets.clone());
    let signals = Rc::new(signals);
    let missing_fields: RwSignal<Vec<MissingField>> = RwSignal::new(Vec::<MissingField>::new());

    let input_data = RwSignal::new(Option::<domain::PlantProfile>::None);

    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let selected_scenario_name = RwSignal::new(String::new());
    let barchart_arguments: RwSignal<Vec<klick_svg_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let current_section = RwSignal::new(Option::<PageSection>::None);
    let n2o_emission_factor_method =
        RwSignal::new(Option::<app::N2oEmissionFactorCalcMethod>::None);

    let nitrogen_io_warning = RwSignal::new(Option::<String>::None);
    let chemical_oxygen_io_warning = RwSignal::new(Option::<String>::None);
    let phosphorus_io_warning = RwSignal::new(Option::<String>::None);

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        let (data, filtered_required_fields) = read_input_fields(&s, &required_fields);
        missing_fields.set(filtered_required_fields);
        input_data.set(data.try_into().ok());
    });

    let custom_factor_value: RwSignal<Option<f64>> = signals
        .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
        .and_then(FieldSignal::get_float_output_signal)
        .unwrap();

    create_effect(move |_| {
        let Some(n) = selected_scenario.get() else {
            n2o_emission_factor_method.set(None);
            return;
        };

        let f = match n {
            0 => app::N2oEmissionFactorCalcMethod::TuWien2016,
            1 => app::N2oEmissionFactorCalcMethod::Optimistic,
            2 => app::N2oEmissionFactorCalcMethod::Pesimistic,
            3 => app::N2oEmissionFactorCalcMethod::Ipcc2019,
            4 => app::N2oEmissionFactorCalcMethod::Custom(domain::Factor::new(
                custom_factor_value.get().unwrap_or_default() / 100.0,
            )),
            _ => {
                n2o_emission_factor_method.set(None);
                return;
            }
        };
        n2o_emission_factor_method.set(Some(f));
    });

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        let mut input_data_validation_error = false;
        if let Some(input_data) = input_data.get() {
            let custom_factor_value = s
                .get(&FieldId::Scenario(ScenarioFieldId::N2oCustomFactor))
                .and_then(FieldSignal::get_float);
            let use_custom_factor = custom_factor_value.is_some();
            if !use_custom_factor && selected_scenario.get() == Some(4) {
                selected_scenario.set(Some(0));
            }

            if input_data.effluent_average.nitrogen > input_data.influent_average.nitrogen {
                nitrogen_io_warning.set(Some(format!(
                    "Ablauf Gesamtstickstoff {} größer als dessen Zulauf {}!",
                    Lng::De.format_number(input_data.effluent_average.nitrogen),
                    Lng::De.format_number(input_data.influent_average.nitrogen)
                )));
                input_data_validation_error = true;
            } else {
                nitrogen_io_warning.set(None);
            }
            if let Some(chemical_oxygen_demand_influent) =
                input_data.influent_average.chemical_oxygen_demand
            {
                if input_data.effluent_average.chemical_oxygen_demand
                    > chemical_oxygen_demand_influent
                {
                    chemical_oxygen_io_warning.set(Some(format!(
                        "Ablauf Chemischer Sauerstoffbedarf {} größer als dessen Zulauf {}!",
                        Lng::De.format_number(input_data.effluent_average.chemical_oxygen_demand),
                        Lng::De.format_number(chemical_oxygen_demand_influent)
                    )));
                    input_data_validation_error = true;
                } else {
                    chemical_oxygen_io_warning.set(None);
                }
            }

            if let Some(phosphorus_influent) = input_data.influent_average.phosphorus {
                if let Some(phosphorus_effluent) = input_data.effluent_average.phosphorus {
                    if phosphorus_effluent > phosphorus_influent {
                        phosphorus_io_warning.set(Some(format!(
                            "Ablauf Phosphor {} größer als dessen Zulauf {}!",
                            Lng::De.format_number(phosphorus_effluent),
                            Lng::De.format_number(phosphorus_influent),
                        )));
                        input_data_validation_error = true;
                    } else {
                        phosphorus_io_warning.set(None);
                    }
                }
            }

            log::debug!("Calculating with {input_data:#?}");
            let szenario_calculations = N2oEmissionFactorCalcMethod::iter()
                .enumerate()
                .filter_map(|(i, method)| {
                    if input_data_validation_error {
                        // prevent sankey or barchart from rendering
                        sankey::clear(CHART_ELEMENT_ID);
                        return None
                    }
                    let n2o_emission_factor = match method {
                        N2oEmissionFactorCalcMethod::CustomFactor => {
                            app::N2oEmissionFactorCalcMethod::Custom(domain::Factor::new(custom_factor_value.unwrap_or_default() / 100.0))
                        }
                        N2oEmissionFactorCalcMethod::TuWien2016 => app::N2oEmissionFactorCalcMethod::TuWien2016,
                        N2oEmissionFactorCalcMethod::Optimistic => app::N2oEmissionFactorCalcMethod::Optimistic,
                        N2oEmissionFactorCalcMethod::Pesimistic => app::N2oEmissionFactorCalcMethod::Pesimistic,
                        N2oEmissionFactorCalcMethod::Ipcc2019 => app::N2oEmissionFactorCalcMethod::Ipcc2019,
                    };

                    let scenario = app::Scenario {
                    n2o_emission_factor,
                    ch4_chp_emission_factor: None,
                 };

                 let output_data = klick_application::calculate_emissions(&input_data, scenario);

                    if selected_scenario.get() == Some(i as u64) {
                        let name_ka: String = s
                            .get(&FieldId::Name)
                            .and_then(FieldSignal::get_text)
                            .unwrap_or_else(|| "Kläranlage".to_string());

                        let ew = s
                            .get(&FieldId::Ew)
                            .and_then(FieldSignal::get_float)
                            .unwrap_or_default();

                        let einheit = "t CO₂ Äquivalente/Jahr";
                        let szenario_name = label_of_n2o_emission_factor_calc_method(&method);
                        selected_scenario_name.set(szenario_name.to_string().clone());
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
                        co2_data: d.co2_equivalents.emissions.into(),
                        n2o_factor: f64::from(d.n2o_emission_factor),
                    })
                    .collect(),
            );
        } else {
            sankey_header.set(String::new());
            barchart_arguments.update(Vec::clear);
            sankey::clear(CHART_ELEMENT_ID);
            nitrogen_io_warning.set(None);
            chemical_oxygen_io_warning.set(None);
            phosphorus_io_warning.set(None);
        }
    });

    let upload_action = create_action({
        let signals = Rc::clone(&signals);
        move |file: &File| {
            let signals = Rc::clone(&signals);
            let file = file.clone();
            async move {
                match gloo_file::futures::read_as_bytes(&file).await {
                    Ok(bytes) => match import_from_slice(&bytes) {
                        Ok((input, scenario)) => {
                            load_fields(&signals, input, scenario);
                        }
                        Err(err) => {
                            log::warn!("Unable to import data: {err}");
                        }
                    },
                    Err(err) => {
                        log::warn!("Unable to upload data: {err}");
                    }
                }
            }
        }
    });

    let clear_signals = {
        let signals = Rc::clone(&signals);
        move || {
            for s in signals.values() {
                s.clear();
            }
        }
    };

    let load_example_values = {
        let signals = Rc::clone(&signals);
        move || {
            example_data::load_example_field_signal_values(&signals);
        }
    };

    let save_input_values = {
        let signals = Rc::clone(&signals);
        move || {
            let (input, _) = read_input_fields(&signals, &vec![]);
            let szenario = read_scenario_fields(&signals);
            let json_bytes = export_to_vec_pretty(&input, &szenario);

            let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));

            ObjectUrl::from(blob)
        }
    };

    create_effect(move |_| {
        if let Some(s) = current_section.get() {
            let id = s.section_id();
            window()
                .location()
                .set_href(&format!("{}#{id}", crate::Page::Tool.path()))
                .unwrap();
        }
    });

    let breadcrumps_entries = vec![
        ("Datenerfassung", PageSection::DataCollection),
        (
            "Auswertung & Handlungsempfehlungen",
            PageSection::OptimizationOptions,
        ),
    ];

    view! {
      <div class="space-y-12">
        <ActionPanel
          clear = clear_signals
          load = load_example_values
          save_project = save_input_values
          upload_action
        />
        <Breadcrumbs
          entries = { breadcrumps_entries }
          current = current_section
        />
        <div
          id = PageSection::DataCollection.section_id()
          class = move || {
            if current_section.get() == Some(PageSection::DataCollection) || current_section.get() == None {
                None
            } else {
                Some("hidden")
            }
          }
        >
         { set_views }
        </div>
        <div
          class = move || {
            if current_section.get() == Some(PageSection::DataCollection) {
                Some("hidden")
            } else {
                None
            }
          }
        >
          <InputDataList
            field_sets = { &field_sets }
            signals = { &signals }
          />
        </div>
      </div>

      { move ||
          if barchart_arguments.get().is_empty() {
              Some(view! {
                <div class="mt-5">
                  <p>"Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage, anhand verschiedener Szenarien, berechnet werden können:"</p>
                    <forms::HelperWidget
                      missing_fields=missing_fields.get()
                      before_focus = move || {
                        current_section.set(Some(PageSection::DataCollection));
                      }
                    />
                  <Show when= move || nitrogen_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ nitrogen_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <Show when= move || chemical_oxygen_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ chemical_oxygen_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <Show when= move || phosphorus_io_warning.get().is_some()>
                    <p>
                      <ul class="ml-5 my-4 list-disc list-inside">
                        <li>{ phosphorus_io_warning.get() }</li>
                      </ul>
                    </p>
                  </Show>
                  <p>"Bei jeder Eingabe werden die Graphen automatisch neu berechnet."</p>
                </div>
              })
          } else {
              None
          }
      }

      <h3 id = PageSection::OptimizationOptions.section_id() class="my-8 text-xl font-bold">
        "Auswertung Ihrer Daten (via Barchart / Sankey-Diagramm)"
      </h3>

      <Show
        when = move || !barchart_arguments.get().is_empty()
        fallback = || view! {
          <p>
            "Bitte ergänzen Sie im Eingabeformular die fehlenden Werte, damit die Emissionen berechnet und visualisiert werden können."
          </p>
        }
      >
        // bar diagram
        <h4 class="my-8 text-lg font-bold">"Szenarien im Vergleich - Treibhausgasemissionen [t CO₂ Äquivalente/Jahr]"</h4>
        <div class="">
          <BarChart
            width = 1200.0
            height = 400.0
            data  = barchart_arguments.into()
            selected_bar = selected_scenario
          />
        </div>
        <p>
          "Es ist das Szenario \"" { selected_scenario_name.get() } "\" ausgewählt.
          Durch Anklicken kann ein anderes Szenario ausgewählt werden."
        </p>

        // sankey diagram
        <h4 class="my-8 text-lg font-bold">
          { move || sankey_header.get().to_string() }
        </h4>
      </Show>

      <div id={ CHART_ELEMENT_ID } class="mt-8"></div>

      <div class="my-8 border-b border-gray-200 pb-5" >
        <h3
          class="text-xl font-semibold leading-6 text-gray-900 cursor-pointer"
          on:click = move |_| {
            current_section.set(Some(PageSection::OptimizationOptions));
          }
        >
          "Minderungsmaßnahmen für THG-Emissionen an Kläranlagen"
        </h3>
        <p class="mt-2 max-w-4xl text-lg text-gray-500">
          "Die vorgestellten Handlungsempfehlungen stellen eine erste Auswahl
          an möglichen Minderungsmaßnahmen für Treibhausgasemissionen (THG) an Kläranlagen dar.
          Diese sollen Ihnen wichtige Mehrwerte bieten, um die Klimaauswirkungen Ihrer Kläranlage zu minimieren
          und deren Wettbewerbsfähigkeit langfristig zu sichern."
        </p>
      </div>
      <OptimizationOptions
        input_data = input_data.into()
        n2o_emission_factor_method = n2o_emission_factor_method.into()
      />
    }
}

const fn label_of_n2o_emission_factor_calc_method(
    method: &N2oEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        N2oEmissionFactorCalcMethod::TuWien2016 => "TU Wien 2016",
        N2oEmissionFactorCalcMethod::Optimistic => "Optimistisch",
        N2oEmissionFactorCalcMethod::Pesimistic => "Pessimistisch",
        N2oEmissionFactorCalcMethod::Ipcc2019 => "IPCC 2019",
        N2oEmissionFactorCalcMethod::CustomFactor => "Benutzerdefiniert",
    }
}
