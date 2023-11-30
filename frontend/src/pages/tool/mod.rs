use std::rc::Rc;

use gloo_file::{Blob, File, ObjectUrl};
use leptos::*;
use strum::IntoEnumIterator;

use klick_application as app;
use klick_boundary::{export_to_vec_pretty, import_from_slice, N2oEmissionFactorCalcMethod};
use klick_svg_charts::BarChart;

use crate::{
    forms::{self, FieldSignal, MissingField},
    sankey,
};

mod action_panel;
mod example_data;
mod field_sets;
mod fields;

use self::{
    action_panel::ActionPanel,
    field_sets::field_sets,
    fields::{load_fields, read_input_fields, read_scenario_fields, FieldId},
};

const CHART_ELEMENT_ID: &str = "chart";

#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool() -> impl IntoView {
    let field_sets = field_sets();
    let (signals, set_views, required_fields) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);
    let missing_fields: RwSignal<Vec<MissingField>> = RwSignal::new(Vec::<MissingField>::new());

    let input_data = RwSignal::new(Option::<app::Input>::None);

    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let selected_scenario_name = RwSignal::new(String::new());
    let barchart_arguments: RwSignal<Vec<klick_svg_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let (data, filtered_required_fields) = read_input_fields(&s, &required_fields);
        missing_fields.set(filtered_required_fields);
        input_data.set(data.try_into().ok());
    });

    let s = Rc::clone(&signals);

    create_effect(move |_| {
        if let Some(input_data) = input_data.get() {
            let custom_factor_value = s
                .get(&FieldId::CustomN2oScenarioValue)
                .and_then(FieldSignal::get_float);
            let use_custom_factor = custom_factor_value.is_some();
            if !use_custom_factor && selected_scenario.get() == Some(4) {
                selected_scenario.set(Some(0));
            }
            log::debug!("Calculating with {input_data:#?}");
            let szenario_calculations = N2oEmissionFactorCalcMethod::iter()
            .enumerate()
            .filter_map(|(i, method)| {

                  let calc_method = match method {
                      N2oEmissionFactorCalcMethod::CustomFactor => {
                          app::N2oEmissionFactorCalcMethod::Custom(app::Factor::new(custom_factor_value.unwrap_or_default() / 100.0))
                      }
                      N2oEmissionFactorCalcMethod::ExtrapolatedParravicini=>  app::N2oEmissionFactorCalcMethod::ExtrapolatedParravicini,
                      N2oEmissionFactorCalcMethod::Optimistic             =>  app::N2oEmissionFactorCalcMethod::Optimistic,
                      N2oEmissionFactorCalcMethod::Pesimistic             =>  app::N2oEmissionFactorCalcMethod::Pesimistic,
                      N2oEmissionFactorCalcMethod::Ipcc2019               =>  app::N2oEmissionFactorCalcMethod::Ipcc2019,
                  };

                 let output_data = klick_application::calculate_emissions(&input_data, calc_method);

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
            let object_url = ObjectUrl::from(blob);
            object_url
        }
    };

    view! {
      <div class="space-y-12">
        <ActionPanel
          clear = clear_signals
          load = load_example_values
          save_project = save_input_values
          upload_action
        />
        { set_views }
      </div>
      // form field requirements helper widget
      { move ||  {
          if barchart_arguments.get().is_empty() {
              Some(view! {

              <div>
                  <h3 class="my-8 text-xl font-bold">"Auswertung Ihrer Daten (via Barchart / Sankey-Diagramm)"</h3>
                  <p>"Bitte ergänzen Sie folgende Werte, damit die Gesamtemissionen Ihrer Kläranlage, anhand verschiedener Szenarien, berechnet werden können:"</p>
                  <forms::HelperWidget missing_fields=missing_fields.get()/>
                  <p>"Bei jeder Eingabe werden die Graphen automatisch neu berechnet."</p>
              </div>
              })
          } else {
              None
          }
          }
      }
      // bar diagram
      { move ||
        {
          if barchart_arguments.get().is_empty() {
            None
          } else {
            Some(view! {
              <h3 class="my-8 text-xl font-bold">"Szenarien im Vergleich - Treibhausgasemissionen [t CO₂ Äquivalente/Jahr]"</h3>
              <div class="">
                <BarChart
                  width = 1200.0
                  height = 400.0
                  data  = barchart_arguments.into()
                  selected_bar = selected_scenario
                />
              </div>
                <p>"Es ist das Szenario \""{selected_scenario_name.get()}"\" ausgewählt. Durch Anklicken kann ein anderes Szenario ausgewählt werden."</p>
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
