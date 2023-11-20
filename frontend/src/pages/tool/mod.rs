use std::{collections::HashMap, rc::Rc};

use gloo_file::{Blob, File, ObjectUrl};
use leptos::{ev::MouseEvent, *};
use strum::IntoEnumIterator;

use klick_application as app;
use klick_boundary::{export_to_vec_pretty, import_from_slice, N2oEmissionFactorCalcMethod};
use klick_svg_charts::BarChart;

use crate::{
    forms::{self, FieldSignal},
    sankey,
};

mod example_data;
mod field_sets;
mod fields;

use self::{
    field_sets::field_sets,
    fields::{load_fields, read_input_fields, read_scenario_fields, FieldId},
};

const CHART_ELEMENT_ID: &str = "chart";

#[component]
#[allow(clippy::too_many_lines)]
pub fn Tool() -> impl IntoView {
    let field_sets = field_sets();

    let (signals, set_views) = forms::render_field_sets(field_sets);
    let signals = Rc::new(signals);

    let input_data = RwSignal::new(Option::<app::Input>::None);

    let sankey_header = RwSignal::new(String::new());
    let selected_scenario = RwSignal::new(Option::<u64>::Some(0));
    let barchart_arguments: RwSignal<Vec<klick_svg_charts::BarChartArguments>> =
        RwSignal::new(vec![]);

    let show_upload_input = RwSignal::new(false);

    let s = Rc::clone(&signals);
    create_effect(move |_| {
        let data = read_input_fields(&s).try_into().ok();
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
                        co2_data: d.co2_equivalents.emissions,
                        n2o_factor: d.n2o_emission_factor,
                    })
                    .collect(),
            );
        } else {
            sankey_header.set(String::new());
            barchart_arguments.update(Vec::clear);
            sankey::clear(CHART_ELEMENT_ID);
        }
    });

    let download_link: NodeRef<leptos::html::A> = create_node_ref();
    let upload_input: NodeRef<leptos::html::Input> = create_node_ref();

    let upload_action = create_action({
        let signals = Rc::clone(&signals);
        move |_: &()| {
            let signals = Rc::clone(&signals);
            async move {
                if let Err(err) = upload_and_load(signals, upload_input).await {
                    log::warn!("Unable to upload data: {err}");
                }
                show_upload_input.set(false);
            }
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
          <Button
            label = "Project speichern"
            on_click = {
              let signals = Rc::clone(&signals);
              move |ev| {
                ev.prevent_default();

                let input = read_input_fields(&signals);
                let szenario = read_scenario_fields(&signals);
                let json_bytes = export_to_vec_pretty(&input, &szenario);

                let blob = Blob::new_with_options(&*json_bytes, Some("application/json"));
                let object_url = ObjectUrl::from(blob);

                let link = download_link.get().expect("<a> to exist");
                link.set_attribute("href", &object_url).unwrap();
                link.set_attribute("download", "klimabilanzklaeranlage.json").unwrap();
                link.click();
                link.remove_attribute("href").unwrap();
              }
            }
          />
          <Button
            label = "Project laden"
            on_click = {
              move |ev| {
                ev.prevent_default();
                show_upload_input.set(true);
              }
            }
          />
          <input
            class = "block text-sm bg-gray-50 rounded-md shadow-sm file:bg-primary file:rounded-md file:border-0 file:mr-4 file:py-1 file:px-2 file:font-semibold"
            type="file"
            style = move || if !show_upload_input.get() { Some("display:none;") } else { None }
            accept="application/json"
            node_ref=upload_input
            on:change = move |ev| {
                ev.prevent_default();
                upload_action.dispatch(());
            }
          />
          // Hidden download anchor
          <a style="display:none;" node_ref=download_link></a>
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

async fn upload_and_load(
    signals: Rc<HashMap<FieldId, FieldSignal>>,
    upload_input: NodeRef<leptos::html::Input>,
) -> anyhow::Result<()> {
    let Some(file_list) = get_file_list(upload_input) else {
        log::debug!("No file list");
        return Ok(());
    };
    let Some(file) = file_list.item(0) else {
        log::debug!("No file selected");
        return Ok(());
    };
    let gloo_file = File::from(file);
    let bytes = gloo_file::futures::read_as_bytes(&gloo_file).await?;
    let (input, scenario) = import_from_slice(&bytes)?;
    load_fields(&signals, input, scenario);
    Ok(())
}

fn get_file_list(upload_input: NodeRef<leptos::html::Input>) -> Option<web_sys::FileList> {
    upload_input.get().expect("<input /> to exist").files()
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
