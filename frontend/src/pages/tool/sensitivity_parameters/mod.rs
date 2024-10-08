use klick_presenter::Lng;
use leptos::*;
use leptos_fluent::*;

use klick_app_charts::{BarChart, BarChartArguments};
use klick_boundary::FormData;

use crate::{
    current_lang,
    pages::tool::{CalculationOutcome, DataCollectionEnforcementHelper, PageSection},
    sankey::Sankey,
};

mod additional_custom_emissions;
mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_open_sludge_storage;
mod fossil_co2_emissions;
mod n2o_emissions;

use self::{
    additional_custom_emissions::*, ch4_emissions_chp::*, ch4_emissions_open_digesters::*,
    ch4_emissions_open_sludge_storage::*, fossil_co2_emissions::*, n2o_emissions::*,
};

#[allow(clippy::too_many_lines)] // TODO
#[component]
pub fn SensitivityParameters(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    sensitivity_outcome: Signal<CalculationOutcome>,
    profile_outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    accessibility_always_show_option: Option<RwSignal<bool>>,
    custom_emissions_message: RwSignal<String>,
    lang: Lng,
) -> impl IntoView {
    let old_output = Memo::new(move |_| profile_outcome.with(|out| out.output.clone()));
    let new_output = Memo::new(move |_| sensitivity_outcome.with(|out| out.output.clone()));

    let barchart_arguments = Memo::new(move |_| {
        old_output
            .get()
            .and_then(|old| new_output.get().map(|new| (new, old)))
            .map(|(new, old)| {
                klick_presenter::sensitivity_diff_bar_chart(old, new, lang)
                    .into_iter()
                    .map(|(label, value, percentage)| BarChartArguments {
                        label,
                        value,
                        percentage,
                    })
                    .collect::<Vec<_>>()
            })
    });

    view! {
      <Show
        when = move || profile_outcome.with(|out|out.output.is_some())
        fallback = move || view!{  <DataCollectionEnforcementHelper current_section /> }
      >
        <div class="my-4 ml-4">
          <h3 class="mt-6 text-lg font-semibold leading-7 text-gray-900">{move_tr!("sensitivity-of-emission-factors")}</h3>
          <p class="my-2">
          {move_tr!("sensitivity-of-emission-factors-info")}
          </p>
        </div>
        <N2OEmissionsSensitivity
          form_data
          sensitivity_outcome
          show_side_stream_controls
          accessibility_always_show_option
          lang
        />
        <CH4EmissionsCHP
          form_data
          sensitivity_outcome
          accessibility_always_show_option
          lang
        />
        <CH4EmissionsOpenDigesters
          form_data
          sensitivity_outcome
          accessibility_always_show_option
          lang
        />
        <CH4EmissionsOpenSludgeStorage
          form_data
          accessibility_always_show_option
        />
        <FossilCO2Emissions
          form_data
          sensitivity_outcome
          accessibility_always_show_option
          lang
        />
        <AdditionalCustomEmissions
          form_data
          sensitivity_outcome
          accessibility_always_show_option
          custom_emissions_message
          lang
        />

        <h4 class="my-8 text-lg font-bold">
          { move || sensitivity_outcome.with(|outcome|outcome.output.as_ref().map(|out|{
                klick_presenter::create_sankey_chart_header(
                  &outcome.input,
                  out.clone(),
                  klick_presenter::Formatting::Text,
                  current_lang().get(),
                )
              }))
          }
        </h4>

        { move || sensitivity_outcome.with(|out| out.output.clone().zip(out.graph.clone()).map(|(data, graph)|{
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
            {move_tr!("sensitivity-barchart-title")}
          </h3>
          <p class="mt-2 max-w-4xl text-lg text-gray-500">
            {move_tr!("sensitivity-barchart-description")}
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
                      aria_label = Some(move_tr!("aria_label_barchart").get())
                      number_format = move |a,b| current_lang().get().format_number_with_fixed_precision(a,b)
                  />
                  }
              }))
            }
          }
        </div>
      </div>
        <button
        class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
        on:click = move |_| { current_section.set(PageSection::Recommendation); }
      >
         { move_tr!("to-the-recommendations") }
      </button>
      </Show>
    }
}
