use leptos::*;
use leptos_fluent::*;

use klick_app_charts::{BarChart, BarChartArguments};
use klick_boundary::FormData;

use crate::{
    current_lang,
    pages::tool::{
        form_data_overview::FormDataOverview, CalculationOutcome, DataCollectionEnforcementHelper,
        PageSection,
    },
    sankey::Sankey,
};

mod ch4_emissions_open_digesters;
mod ch4_emissions_pre_treatment;
mod excess_energy_co2_equivalent;
mod leak_test;
mod n2o_emissions_in_the_biological_treatment_stage;
mod n2o_emissions_side_stream_system;

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn Recommendations(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    recommendation_outcome: Signal<CalculationOutcome>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
) -> impl IntoView {
    let lang = crate::current_lang();

    let old_output = Memo::new(move |_| sensitivity_outcome.with(|out| out.output.clone()));
    let new_output = Memo::new(move |_| recommendation_outcome.with(|out| out.output.clone()));

    let barchart_arguments = Memo::new(move |_| {
        old_output
            .get()
            .and_then(|old| new_output.get().map(|new| (new, old)))
            .map(|(new, old)| {
                klick_presenter::sensitivity_diff_bar_chart(old, new, lang.get())
                    .into_iter()
                    .map(|(label, value, percentage)| BarChartArguments {
                        label,
                        value,
                        percentage,
                    })
                    .collect::<Vec<_>>()
            })
    });

    let form_data_overview = move || {
        recommendation_outcome.with(|out| {
            let input = out.input.clone();
            view! {
              <FormDataOverview input />
            }
        })
    };

    view! {
      <Show
        when = move || recommendation_outcome.with(|out|out.output.is_some())
        fallback = move || view!{  <DataCollectionEnforcementHelper current_section /> }
      >
      <h4 class="my-8 text-lg font-bold">
      { move_tr!("form_data_table_overview")}

      </h4>
      { form_data_overview }
      { n2o_emissions_in_the_biological_treatment_stage::options(
        accessibility_always_show_option
      ) }
      {
        n2o_emissions_side_stream_system::options(
          form_data,
          form_data.into(),
          recommendation_outcome,
          show_side_stream_controls,
          accessibility_always_show_option
        )
      }
      { ch4_emissions_pre_treatment::options(
        accessibility_always_show_option
      ) }
      { ch4_emissions_open_digesters::options(
          form_data,
          form_data.into(),
          recommendation_outcome,
          accessibility_always_show_option,
      ) }
      { leak_test::options(
        accessibility_always_show_option
      ) }
      { excess_energy_co2_equivalent::options(
          form_data,
          form_data.into(),
          recommendation_outcome,
          accessibility_always_show_option,
        )
      }

      <h4 class="my-8 text-lg font-bold">
        { move || recommendation_outcome.with(|outcome|outcome.output.as_ref().map(|out|{
              klick_presenter::create_sankey_chart_header(
                &outcome.input,
                out.clone(),
                klick_presenter::Formatting::Text,
                current_lang().get(),
              )
            }))
        }
      </h4>

      { move || recommendation_outcome.with(|out| out.output.clone().zip(out.graph.clone()).map(|(data, graph)|{
          let lang = current_lang().get();
          view!{ <Sankey data graph lang/> }
        }))
      }
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
            "Änderungen durch Optionen der Handlungsmaßnahmen"
          </h3>
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            "Die folgende Grafik zeigt die Änderungen der Treibhausgasemissionen [t CO₂ Äquivalente/Jahr] bzw. % der Gesamtemissionen durch die ausgewählten Handlungsmaßnahmen."
          </p>
          { move || {
              let lang = current_lang().get();
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
                      aria_label = Some("Ein Balkendiagramm innerhalb der Handlungsoptionen, welches nur angezeigt wird, wenn eine Verbesserung / Verschlechterung durch eine Auswahl eingetreten ist.".to_string())
                      number_format = move |a,b| lang.format_number_with_fixed_precision(a,b)
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
