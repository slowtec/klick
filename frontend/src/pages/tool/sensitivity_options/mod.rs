use klick_domain as domain;
use leptos::*;

mod ch4_emissions_chp;
pub mod ch4_emissions_open_digesters;
mod ch4_emissions_open_sludge_storage;
mod fossil_co2_emissions;
pub mod n2o_emissions;

use crate::pages::tool::widgets::{Card, Cite, InfoBox, ScenarioHint, DWA_MERKBLATT_URL};

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
    n2o_side_stream: RwSignal<Option<f64>>,
    custom_factor_n2o: RwSignal<Option<f64>>,
    co2_fossil_custom_factor: RwSignal<Option<f64>>,
    sludge_bags_are_open: RwSignal<Option<bool>>,
    sludge_storage_containers_are_open: RwSignal<Option<bool>>,
    custom_sludge_bags_factor: RwSignal<Option<f64>>,
    custom_sludge_storage_containers_factor: RwSignal<Option<f64>>,
) -> impl IntoView {
    view! {
      { n2o_emissions::options(output, barchart_arguments_radio_inputs, selected_scenario_name_n2o, selected_scenario_n2o, custom_factor_n2o, n2o_side_stream ) }
      { ch4_emissions_chp::options(output, selected_scenario_chp, selected_scenario_name_chp, custom_factor_bhkw, barchart_arguments_radio_inputs_bhkw) }
      { ch4_emissions_open_digesters::options(output, sludge_bags_are_open, custom_sludge_bags_factor, sludge_storage_containers_are_open, custom_sludge_storage_containers_factor) }
      { ch4_emissions_open_sludge_storage::options() }
      { fossil_co2_emissions::options(co2_fossil_custom_factor) }
    }
}
