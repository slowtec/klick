use crate::forms::InfoIcon;
use klick_application as app;
use leptos::*;

mod ch4_emissions_chp;
mod ch4_emissions_open_digesters;
mod ch4_emissions_pre_treatment;
mod excess_energy_co2_equivalent;

const DWA_MERKBLATT_URL: &str =
    "https://shop.dwa.de/DWA-M-230-1-Treibhausgasemissionen-10-2022/M-230-T1-22";

#[component]
pub fn OptimizationOptions(
    input_data: Signal<Option<app::Input>>,
    n2o_emission_factor_method: Signal<Option<app::N2oEmissionFactorCalcMethod>>,
) -> impl IntoView {
    view! {
      { excess_energy_co2_equivalent::options(input_data, n2o_emission_factor_method) }
      { ch4_emissions_pre_treatment::options() }
      { ch4_emissions_chp::options(input_data, n2o_emission_factor_method) }
      { ch4_emissions_open_digesters::options(input_data, n2o_emission_factor_method) }
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
    let hide = RwSignal::<bool>::new(false);
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
            class = move || if hide.get() { "w-3 h-3 shrink-0" } else { "w-3 h-3 rotate-180 shrink-0" }
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
    output: Signal<Option<app::Output>>,
    n2o_emission_factor_method: Signal<Option<app::N2oEmissionFactorCalcMethod>>,
) -> impl IntoView {
    move || {
        n2o_emission_factor_method.get().and_then(|x| {
            output.get().map(|out| {
                let f = f64::from(out.n2o_emission_factor) * 100.0;
                let ef = format!("(N₂O EF = {f:.2}%)");

                let scenario = match x {
                    app::N2oEmissionFactorCalcMethod::ExtrapolatedParravicini => {
                        format!("Extrapoliert {ef}")
                    }
                    app::N2oEmissionFactorCalcMethod::Optimistic => format!("Optimistisch {ef}"),
                    app::N2oEmissionFactorCalcMethod::Pesimistic => format!("Pesimistisch {ef}"),
                    app::N2oEmissionFactorCalcMethod::Ipcc2019 => format!("IPCC 2019 {ef}"),
                    app::N2oEmissionFactorCalcMethod::Custom(f) => {
                        format!("Benutzerdefiniert ({:.1})", f64::from(f) * 100.0)
                    }
                };

                view! {
                   <p>
                     "Bezogen auf das Szenario " { scenario }":"
                   </p>
                }
            })
        })
    }
}
