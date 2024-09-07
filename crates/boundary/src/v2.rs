use serde::Deserialize;

pub use crate::v1::{
    AnnualAverage, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    Scenario, SewageSludgeTreatment,
};

#[derive(Deserialize)]
pub struct Import {
    pub input: InputData,
    pub scenario: Scenario,
}

#[derive(Deserialize)]
pub struct InputData {
    pub plant_name: Option<String>,
    pub population_equivalent: Option<f64>,
    pub wastewater: Option<f64>,
    pub influent_average: AnnualAverage,
    pub effluent_average: AnnualAverage,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub operating_materials: OperatingMaterials,
}

#[derive(Deserialize)]
pub struct EnergyConsumption {
    pub sewage_gas_produced: Option<f64>,
    pub methane_fraction: Option<f64>,
    pub gas_supply: Option<f64>,
    pub purchase_of_biogas: Option<bool>,
    pub total_power_consumption: Option<f64>,
    pub on_site_power_generation: Option<f64>,
    pub emission_factor_electricity_mix: Option<f64>,
}
