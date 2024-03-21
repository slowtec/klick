use leptos::*;

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

#[component]
pub fn SensitivityParameters(
    form_data: RwSignal<FormData>,
    current_section: RwSignal<PageSection>,
    outcome: Signal<Option<CalculationOutcome>>,
    show_side_stream_controls: Signal<bool>,
) -> impl IntoView {
    view! {
      <Show
        when = move || outcome.with(|o|o.is_some())
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
          { move || outcome.with(|out|out.as_ref().map(|out|{
                let out = &out.sensitivity.output;
                klick_presenter::create_sankey_chart_header(
                  &form_data.with(|d| d.plant_profile.clone()),
                  out.emission_factors,
                  out.calculation_methods,
                )
              }))
          }
        </h4>

        { move || outcome.with(|out| out.as_ref().map(|outcome|{
            let outcome = outcome.sensitivity.output.clone();
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
      </Show>
    }
}
