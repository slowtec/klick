use leptos::*;

use klick_app_charts::{BarChart, BarChartArguments};
use klick_boundary::FormData;

use crate::{
    pages::tool::{CalculationOutcome, DataCollectionEnforcementHelper, PageSection},
    sankey::Sankey,
};

mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_open_sludge_storage;
mod fossil_co2_emissions;
mod n2o_emissions;

use self::{
    ch4_emissions_chp::*, ch4_emissions_open_digesters::*, ch4_emissions_open_sludge_storage::*,
    fossil_co2_emissions::*, n2o_emissions::*,
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn SensitivityParameters(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let barchart_arguments = create_memo(move |_| {
        outcome.with(|out| {
            // TODO: avoid clones
            out.output
                .as_ref()
                .map(|o| o.co2_equivalents.clone())
                .and_then(|old| {
                    out.output
                        .as_ref()
                        .map(|o| (o.co2_equivalents.clone(), old))
                })
                .map(|(new, old)| {
                    klick_presenter::sensitivity_diff_bar_chart(old, new)
                        .into_iter()
                        .map(|(label, value, percentage)| BarChartArguments {
                            label,
                            value,
                            percentage,
                        })
                        .collect::<Vec<_>>()
                })
        })
    });
    view! {
      <Show
        when = move || outcome.with(|out|out.output.is_some())
        fallback = move || view!{  <DataCollectionEnforcementHelper current_section /> }
      >
        <div class="my-4 ml-4">
        <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">"Sensitivität von Emissionsfaktoren"</h3>
        <p class="my-2">
          "Unter nachfolgenden „aufklappbaren“ Abschnitten haben Sie die Möglichkeit verschiedene Emissionsfaktoren (EF)
          genauer zu definieren. Dabei können Sie berechnen, wie sich die jeweilige Anpassung der EF von
          Anlagenkomponenten bzw. der Gesamtkläranlage auf die Klimabilanz auswirkt. Sie können die
          Sensibilisierung/Verfeinerung auch überspringen und direkt zu den Handlungsempfehlungen übergehen
          (in diesem Fall rechnet das KlicK-Tool auf Basis der genannten Standardfaktoren/-parameter)."
        </p>
        </div>
        <N2OEmissionsSensitivity
          form_data
          outcome
          show_side_stream_controls
          accessibility_always_show_option
        />
        <CH4EmissionsCHP
          form_data
          input_data = form_data.read_only()
          outcome
          accessibility_always_show_option
        />
        <CH4EmissionsOpenDigesters
          form_data
          input_data = form_data.read_only()
          outcome
          accessibility_always_show_option
        />
        <CH4EmissionsOpenSludgeStorage
          form_data
          accessibility_always_show_option
        />
        <FossilCO2Emissions
          form_data
          input_data = form_data.read_only()
          outcome
          accessibility_always_show_option
        />

        <h4 class="my-8 text-lg font-bold">
          { move || outcome.with(|out|out.output.as_ref().map(|out|{
                klick_presenter::create_sankey_chart_header(
                  &form_data.with(Clone::clone), // TODO: avoid clone
                  out.emission_factors,
                  out.calculation_methods,
                  klick_presenter::Formatting::Text,
                )
              }))
          }
        </h4>

        { move || outcome.with(|out| out.output.as_ref().map(|outcome|{
            let data = (outcome.co2_equivalents.clone(), outcome.emission_factors);
            view!{ <Sankey data /> }
          }))
        }

        <button
          class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
          on:click = move |_| { current_section.set(PageSection::Recommendation); }
        >
          "zur den Handlungsempfehlungen"
        </button>
        <div
        class = move || {
          if barchart_arguments.with(|args|args.as_ref().map(|args|args.iter().any(|x| f64::abs(x.value) > 0.1))).unwrap_or(false) {
              None
          } else {
              Some("hidden")
          }
        }
      >
        <div class="mx-auto p-8" >
          <h3 class="text-xl font-semibold leading-6 text-gray-900">
            "Änderungen durch Angaben der Sensitivität"
          </h3>
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            "Das folgende Diagramm zeigt die Änderungen der Treibhausgasemissionen [t CO₂ Äquivalente/Jahr] bzw. die [%]-Änderung der Gesamtemissionen durch die ausgewählten Emissionsfaktoren."
          </p>
          { move || {
              barchart_arguments.with(|args|args.as_ref().map(|arguments|{
                  let barchart_arguments_filtered = arguments
                    .iter()
                    .filter_map(|x| {
                        if f64::abs(x.value) > 0.1 {
                            Some(x.clone())
                        } else {
                            None
                        }
                    }).collect();
                  view! {
                  <BarChart
                      width = 1100.0
                      height = 400.0
                      data=barchart_arguments_filtered
                      aria_label = Some("Ein Balkendiagramm innerhalb der Sensitivität, welches nur angezeigt wird, wenn eine Verbesserung / Verschlechterung durch eine Auswahl eingetreten ist.".to_string())
                  />
                  }
              }))
            }
          }
        </div>
      </div>
      </Show>
    }
}
