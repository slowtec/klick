use leptos::*;

use klick_domain as domain;

use crate::forms::InfoIcon;

mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_open_sludge_storage;
mod fossil_co2_emissions;
pub mod n2o_emissions;

const DWA_MERKBLATT_URL: &str =
    "https://shop.dwa.de/DWA-M-230-1-Treibhausgasemissionen-10-2022/M-230-T1-22";

#[component]
pub fn SensitivityOptions(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    selected_scenario_n2o: RwSignal<Option<u64>>,
    selected_scenario_chp: RwSignal<Option<u64>>,
    custom_factor_bhkw: RwSignal<Option<f64>>,
    barchart_arguments_radio_inputs: ReadSignal<Vec<klick_app_charts::BarChartRadioInputArguments>>,
    barchart_arguments_radio_inputs_bhkw: ReadSignal<
        Vec<klick_app_charts::BarChartRadioInputArguments>,
    >,
    selected_scenario_name_n2o: RwSignal<String>,
    selected_scenario_name_chp: RwSignal<String>,
    custom_factor_n2o: RwSignal<Option<f64>>,
) -> impl IntoView {
    view! {
      { n2o_emissions::options(output, barchart_arguments_radio_inputs, selected_scenario_name_n2o, selected_scenario_n2o, custom_factor_n2o) }
      { ch4_emissions_chp::options(output, selected_scenario_chp, selected_scenario_name_chp, custom_factor_bhkw, barchart_arguments_radio_inputs_bhkw) }
      { ch4_emissions_open_digesters::options() }
      { ch4_emissions_open_sludge_storage::options() }
      { fossil_co2_emissions::options() }
    }
}

// FIXME dup of optimization_options
#[component]
fn InfoBox(text: &'static str, children: Children) -> impl IntoView {
    let show = RwSignal::<bool>::new(false);
    let children = children();

    view! {
      <p>{ text }
        <div
          class="mx-1 cursor-pointer inline-block"
          on:click = move |_| show.update(|x|*x = !*x)
        >
          <InfoIcon />
        </div>
      </p>
      <div class = move || if show.get() { None } else { Some("hidden") } >
        { children }
      </div>
    }
}

#[component]
fn Card(title: &'static str, children: Children) -> impl IntoView {
    let hide = RwSignal::<bool>::new(true);
    let children = children();

    view! {
      <div
        class="mt-8 divide-y divide-gray-200 overflow-hidden rounded-lg bg-white shadow-md"
      >
        <div
          class="px-4 py-3 cursor-pointer flex items-center justify-between" style="background-color: #0af;"
          on:click = move |_| hide.update(|h| *h = !*h)
        >
          <h3 class="font-bold text-lg">{ title }</h3>
          <svg
            class = move || if hide.get() { "w-3 h-3 rotate-180 shrink-0" } else { "w-3 h-3 shrink-0" }
            aria-hidden="true"
            fill="none"
            viewBox="0 0 10 6"
          >
            <path
              stroke="currentColor"
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M9 5 5 1 1 5"
            />
          </svg>
        </div>
        <div
          class = move || if hide.get() { "hidden" } else { "px-4 py-4 sm:px-6 text-md" }
        >
          { children }
        </div>
      </div>
    }
}

#[component]
fn Cite(source: &'static str, url: &'static str, children: Children) -> impl IntoView {
    view! {
      <p class="mt-4 mb-2 mx-3 px-3 border-solid border-l-8 border-slate-50 bg-slate-50 italic">
        { children() }
        <span class="block mt-2 mb-3 not-italic text-right text-sm font-mono">
          <a target="_blank" href = {url} >{ source }</a>
        </span>
      </p>
    }
}

#[component]
fn ScenarioHint(output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>) -> impl IntoView {
    move || {
        output.get().map(|out| {
            let f = f64::from(out.emission_factors.n2o) * 100.0;
            let ef = format!("(N₂O EF = {f:.2}%");

            // TODO: use presenter
            let scenario = match out.calculation_methods.n2o {
                domain::N2oEmissionFactorCalcMethod::TuWien2016 => {
                    format!("TU Wien 2016 {ef}")
                }
                domain::N2oEmissionFactorCalcMethod::Optimistic => format!("Optimistisch {ef}"),
                domain::N2oEmissionFactorCalcMethod::Pesimistic => format!("Pesimistisch {ef}"),
                domain::N2oEmissionFactorCalcMethod::Ipcc2019 => format!("IPCC 2019 {ef}"),
                domain::N2oEmissionFactorCalcMethod::Custom(f) => {
                    format!("Benutzerdefiniert (N₂O EF = {:.2}", f64::from(f) * 100.0)
                }
            };

            view! {
               <p>
                 "Bezogen auf das Szenario " { scenario } ", CH₄ EF = " {
                  // TODO: use presenter
                  format!("{:.2}%", f64::from(out.emission_factors.ch4) * 100.0)
                 } ")"
               </p>
            }
        })
    }
}
