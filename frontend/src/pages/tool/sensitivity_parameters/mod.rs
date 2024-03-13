use leptos::*;

use klick_boundary::FormData;

use crate::{
    pages::tool::{CalculationOutcome, PageSection},
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

#[component]
pub fn SensitivityParameters(
    form_data: RwSignal<FormData>,
    input_data: ReadSignal<FormData>,
    current_section: RwSignal<PageSection>,
    outcome: Signal<Option<CalculationOutcome>>,
    show_side_stream_controls: Signal<bool>,
) -> impl IntoView {
    view! {
      <N2OEmissionsSensitivity
        form_data
        input_data
        outcome
        show_side_stream_controls
      />
      <CH4EmissionsCHP
        form_data
        input_data
        outcome
      />
      <CH4EmissionsOpenDigesters
        form_data
        input_data
        outcome
      />
      <CH4EmissionsOpenSludgeStorage />
      <FossilCO2Emissions
        form_data
        input_data
        outcome
      />

      { move || outcome.with(|out| out.as_ref().map(|outcome|{
          let outcome = outcome.sensitivity.clone();
          let data = (outcome.co2_equivalents, outcome.emission_factors);
          view!{ <Sankey data /> }
        }))
      }

      <button
        class="rounded bg-primary px-2 py-1 text-sm font-semibold text-black shadow-sm"
        on:click = move |_| { current_section.set(PageSection::Recommendation); }
      >
        "zur den Handlungsempfehlungen"
      </button>
    }
}
