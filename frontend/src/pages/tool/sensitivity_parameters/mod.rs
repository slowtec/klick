use leptos::*;

use klick_boundary::FormData;

use crate::{
    pages::tool::{CalculationOutcome, DataCollectionEnforcementHelper, PageSection},
    sankey::Sankey,
};
use klick_app_charts::BarChart;

mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_open_sludge_storage;
mod fossil_co2_emissions;
mod n2o_emissions;

use self::{
    ch4_emissions_chp::*, ch4_emissions_open_digesters::*, ch4_emissions_open_sludge_storage::*,
    fossil_co2_emissions::*, n2o_emissions::*,
};

#[component]
pub fn SensitivityParameters(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    outcome: Signal<CalculationOutcome>,
    show_side_stream_controls: Signal<bool>,
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

                    let n2oy = f64::from(diff.n2o_emissions);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "N₂O Emissionen",
                        value: n2oy,
                        percentage: Some(n2oy / f64::from(new.total_emissions) * 100.0),
                    });

                    let sludgy = f64::from(diff.ch4_sludge_bags);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Schlammtasche",
                        value: sludgy,
                        percentage: Some(sludgy / f64::from(new.total_emissions) * 100.0),
                    });

                    let schlammy = f64::from(diff.ch4_sludge_storage_containers);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Schlammlagerung",
                        value: schlammy,
                        percentage: Some(schlammy / f64::from(new.total_emissions) * 100.0),
                    });

                    let ch4_plant = f64::from(diff.ch4_plant);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ Anlage (unspez.)",
                        value: ch4_plant,
                        percentage: Some(ch4_plant / f64::from(new.total_emissions) * 100.0),
                    });

                    let bhkwy = f64::from(diff.ch4_combined_heat_and_power_plant);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "CH₄ BHKW",
                        value: bhkwy,
                        percentage: Some(bhkwy / f64::from(new.total_emissions) * 100.0),
                    });

                    let fossily = f64::from(diff.fossil_emissions);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "Fossile CO₂",
                        value: fossily,
                        percentage: Some(fossily / f64::from(new.total_emissions) * 100.0),
                    });

                    let neb_stromi = f64::from(diff.n2o_side_stream);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "N₂O Prozesswasser",
                        value: neb_stromi,
                        percentage: Some(neb_stromi / f64::from(new.total_emissions) * 100.0),
                    });

                    let emissionsy = f64::from(diff.total_emissions);
                    comp.push(klick_app_charts::BarChartArguments {
                        label: "Emissionen",
                        value: emissionsy,
                        percentage: Some(emissionsy / f64::from(new.total_emissions) * 100.0),
                    });

                    //if missing_fields.get().len() > 0 {
                    //    log::info!("NOT computing final output data, missing fields");
                    //    show_handlungsempfehlungen.set(false);
                    //} else {
                    //    show_handlungsempfehlungen.set(true);
                    //}

                    comp
                })
        })
    });
    view! {
      <Show
        when = move || outcome.with(|out|out.sensitivity.output.is_some())
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
        />
        <CH4EmissionsCHP
          form_data
          input_data = form_data.read_only()
          outcome
        />
        <CH4EmissionsOpenDigesters
          form_data
          input_data = form_data.read_only()
          outcome
        />
        <CH4EmissionsOpenSludgeStorage
          form_data
        />
        <FossilCO2Emissions
          form_data
          input_data = form_data.read_only()
          outcome
        />

        <h4 class="my-8 text-lg font-bold">
          { move || outcome.with(|out|out.sensitivity.output.as_ref().map(|out|{
                klick_presenter::create_sankey_chart_header(
                  &form_data.with(|d| d.plant_profile.clone()),
                  out.emission_factors,
                  out.calculation_methods,
                  klick_presenter::Formatting::Text,
                )
              }))
          }
        </h4>

        { move || outcome.with(|out| out.sensitivity.output.as_ref().map(|outcome|{
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
