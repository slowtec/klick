use crate::v2;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Import {
    pub input: InputData,
    pub scenario: v2::Scenario,
}

#[derive(Deserialize)]
pub struct InputData {
    pub plant_name: Option<String>,
    pub population_values: Option<f64>,
    pub waste_water: Option<f64>,
    pub inflow_averages: v2::AnnualAverage,
    pub effluent_averages: v2::AnnualAverage,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: v2::SewageSludgeTreatment,
    pub operating_materials: v2::OperatingMaterials,
}

#[derive(Deserialize)]
pub struct EnergyConsumption {
    pub sewage_gas_produced: Option<f64>,
    pub methane_level: Option<f64>,
    pub gas_supply: Option<f64>,
    pub purchase_of_biogas: Option<bool>,
    pub total_power_consumption: Option<f64>,
    pub in_house_power_generation: Option<f64>,
    pub emission_factor_electricity_mix: Option<f64>,
}
