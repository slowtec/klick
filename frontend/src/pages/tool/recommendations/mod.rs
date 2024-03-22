use leptos::*;

use klick_app_charts::BarChart;
use klick_boundary::FormData;

use crate::{
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

#[component]
pub fn Recommendations(
    form_data: RwSignal<FormData>,
    outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
    current_section: RwSignal<PageSection>,
) -> impl IntoView {
    let barchart_arguments = create_memo(move |_| {
        outcome.with(|out| {
            // TODO: avoid clones
            out.sensitivity
                .output
                .as_ref()
                .map(|o| o.co2_equivalents.clone())
                .and_then(|old| {
                    out.recommendation
                        .output
                        .as_ref()
                        .map(|o| (o.co2_equivalents.clone(), old))
                })
                .map(|(new, old)| {
                    let diff = new.clone() - old;

                    let mut comp = vec![];

                    let sludgy = f64::from(diff.ch4_sludge_bags);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Schlupf Schlammtasche",
                        value: sludgy,
                        percentage: Some(sludgy / f64::from(new.total_emissions) * 100.0),
                    });

                    let schlammy = f64::from(diff.ch4_sludge_storage_containers);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Schlupf Schlammlagerung",
                        value: schlammy,
                        percentage: Some(schlammy / f64::from(new.total_emissions) * 100.0),
                    });

                    let ch4_plant = f64::from(diff.ch4_plant);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Anlage (unspez.)",
                        value: ch4_plant,
                        percentage: Some(ch4_plant / f64::from(new.total_emissions) * 100.0),
                    });

                    let neb_stromi = f64::from(diff.n2o_side_stream);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "N₂O Prozesswasserbehandlung",
                        value: neb_stromi,
                        percentage: Some(neb_stromi / f64::from(new.total_emissions) * 100.0),
                    });

                    let emissionsy = f64::from(diff.total_emissions);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "Emissionen",
                        value: emissionsy,
                        percentage: Some(emissionsy / f64::from(new.total_emissions) * 100.0),
                    });

                    comp
                })
        })
    });

    let form_data_overview = move || {
        outcome.with(|out| {
            view! {
              <FormDataOverview
                evaluation_data = out.sensitivity.clone()
              />
            }
        })
    };

    view! {
      <Show
        when = move || outcome.with(|out|out.recommendation.output.is_some())
        fallback = move || view!{  <DataCollectionEnforcementHelper current_section /> }
      >
      <h4 class="my-8 text-lg font-bold">
      "Übersicht über Eingabewerte (Datenerfassung und Sensitivität)"
      </h4>
      { form_data_overview }
      { n2o_emissions_in_the_biological_treatment_stage::options() }
      {
        n2o_emissions_side_stream_system::options(
          form_data,
          form_data.read_only(),
          outcome,
          show_side_stream_controls
        )
      }
      { ch4_emissions_pre_treatment::options() }
      { ch4_emissions_open_digesters::options(
          form_data,
          form_data.read_only(),
          outcome,
      ) }
      { leak_test::options() }
      { excess_energy_co2_equivalent::options(
          form_data,
          form_data.read_only(),
          outcome,
        )
      }

      <h4 class="my-8 text-lg font-bold">
        { move || outcome.with(|out|out.recommendation.output.as_ref().map(|out|{
              klick_presenter::create_sankey_chart_header(
                &form_data.with(|d| d.plant_profile.clone()),
                out.emission_factors,
                out.calculation_methods,
                klick_presenter::Formatting::Text
              )
            }))
        }
      </h4>

      { move || outcome.with(|out| out.recommendation.output.as_ref().map(|output|{
          let data = (output.co2_equivalents.clone(), output.emission_factors);
          view!{ <Sankey data /> }
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
