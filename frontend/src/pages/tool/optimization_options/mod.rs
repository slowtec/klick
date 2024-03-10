use klick_domain as domain;
use leptos::*;

pub mod ch4_emissions_open_digesters;
mod ch4_emissions_pre_treatment;
mod excess_energy_co2_equivalent;
mod leak_test;
mod n2o_emissions_in_the_biological_treatment_stage;
mod n2o_emissions_side_stream_system;

use crate::pages::tool::widgets::{Card, Cite, InfoBox, DWA_MERKBLATT_URL};

#[component]
pub fn OptimizationOptions(
    output: ReadSignal<Option<domain::EmissionsCalculationOutcome>>,
    show_sludge_bags_controls: RwSignal<bool>,
    show_sludge_storage_containers_controls: RwSignal<bool>,
    sludge_bags_are_open_recommendation: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open_recommendation: RwSignal<Option<bool>>,
    n2o_side_stream_cover_is_open: RwSignal<Option<bool>>,
    show_side_stream_controls: RwSignal<bool>,
    process_energy_savings: RwSignal<Option<f64>>,
    fossil_energy_savings: RwSignal<Option<f64>>,
    district_heating: RwSignal<Option<f64>>,
    photovoltaic_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_photovoltaic_usage: RwSignal<Option<f64>>,
    wind_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_wind_energy_usage: RwSignal<Option<f64>>,
    water_energy_expansion: RwSignal<Option<f64>>,
    estimated_self_water_energy_usage: RwSignal<Option<f64>>,
) -> impl IntoView {
    log::info!("OptimizationOptions rendering");
    view! {
      { n2o_emissions_in_the_biological_treatment_stage::options() }
      { n2o_emissions_side_stream_system::options(output, n2o_side_stream_cover_is_open, show_side_stream_controls) }
      { ch4_emissions_pre_treatment::options() }
      { ch4_emissions_open_digesters::options(output, show_sludge_bags_controls, show_sludge_storage_containers_controls, sludge_bags_are_open_recommendation, sludge_storage_containers_are_open_recommendation ) }
      { leak_test::options() }
      { excess_energy_co2_equivalent::options(output, process_energy_savings, fossil_energy_savings, district_heating, photovoltaic_energy_expansion, estimated_self_photovoltaic_usage, wind_energy_expansion, estimated_self_wind_energy_usage, water_energy_expansion, estimated_self_water_energy_usage) }
    }
}
