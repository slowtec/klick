use leptos::*;

use klick_domain as domain;

use crate::forms::InfoIcon;

mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_pre_treatment;
mod excess_energy_co2_equivalent;
mod leak_test;
mod n2o_emissions_in_the_biological_treatment_stage;

const DWA_MERKBLATT_URL: &str =
    "https://shop.dwa.de/DWA-M-230-1-Treibhausgasemissionen-10-2022/M-230-T1-22";

#[component]
pub fn OptimizationOptions(
    output: ReadSignal<
        Option<(
            domain::CO2Equivalents,
            domain::EmissionFactors,
            domain::EmissionFactorCalculationMethods,
        )>,
    >,
    ch4_chp_emission_factor: RwSignal<Option<domain::CH4ChpEmissionFactorCalcMethod>>,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
) -> impl IntoView {
    log::info!("OptimizationOptions rendering");
    view! {
      { n2o_emissions_in_the_biological_treatment_stage::options() }
      { ch4_emissions_pre_treatment::options() }
      { ch4_emissions_chp::options(output, ch4_chp_emission_factor) }
      { ch4_emissions_open_digesters::options(output, sludge_bags_are_open, sludge_storage_containers_are_open) }
      { excess_energy_co2_equivalent::options(output) }
      { leak_test::options() }
    }
}

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
          class="px-4 py-3 bg-yellow cursor-pointer flex items-center justify-between"
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
fn ScenarioHint(
    output: ReadSignal<
        Option<(
            domain::CO2Equivalents,
            domain::EmissionFactors,
            domain::EmissionFactorCalculationMethods,
        )>,
    >,
) -> impl IntoView {
    move || {
        output.get().map(|out| {
            let f = f64::from(out.1.n2o) * 100.0;
            let ef = format!("(N₂O EF = {f:.2}%");

            let scenario = match out.2.n2o {
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
                  format!("{:.2}%", f64::from(out.1.ch4) * 100.0)
                 } ")"
               </p>
            }
        })
    }
}
