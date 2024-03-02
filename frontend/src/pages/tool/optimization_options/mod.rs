use klick_domain as domain;
use leptos::*;

pub mod ch4_emissions_open_digesters;
mod ch4_emissions_pre_treatment;
mod excess_energy_co2_equivalent;
mod leak_test;
mod n2o_emissions_in_the_biological_treatment_stage;
mod n2o_emissions_side_stream_system;

use crate::pages::tool::widgets::{Card, Cite, InfoBox, ScenarioHint, DWA_MERKBLATT_URL};

#[component]
pub fn OptimizationOptions(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
    custom_sludge_bags_factor: RwSignal<Option<f64>>,
    custom_sludge_storage_containers_factor: RwSignal<Option<f64>>,
    n2o_side_stream_cover_is_open: RwSignal<Option<bool>>,
) -> impl IntoView {
    log::info!("OptimizationOptions rendering");
    view! {
      { n2o_emissions_in_the_biological_treatment_stage::options() }
      { n2o_emissions_side_stream_system::options(n2o_side_stream_cover_is_open) }
      { ch4_emissions_pre_treatment::options() }
      { ch4_emissions_open_digesters::options(output, sludge_bags_are_open, custom_sludge_bags_factor, sludge_storage_containers_are_open, custom_sludge_storage_containers_factor) }
      { leak_test::options() }
      { excess_energy_co2_equivalent::options(output) }
    }
}
