use serde::Deserialize;

#[derive(Deserialize)]
pub struct Import {
    pub input: InputData,
    pub scenario: Scenario,
}

#[derive(Deserialize)]
pub struct InputData {
    pub plant_name: Option<String>,
    pub population_values: Option<f64>,
    pub waste_water: Option<f64>,
    pub inflow_averages: AnnualAverage,
    pub effluent_averages: AnnualAverage,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub operating_materials: OperatingMaterials,
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

#[derive(Deserialize)]
pub struct AnnualAverage {
    pub nitrogen: Option<f64>,
    pub chemical_oxygen_demand: Option<f64>,
    pub phosphorus: Option<f64>,
}

#[derive(Deserialize)]
pub struct SewageSludgeTreatment {
    pub open_sludge_bags: Option<bool>,
    pub open_sludge_storage_containers: Option<bool>,
    pub sewage_sludge_for_disposal: Option<f64>,
    pub transport_distance: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct OperatingMaterials {
    pub fecl3: Option<f64>,
    pub feclso4: Option<f64>,
    pub caoh2: Option<f64>,
    pub synthetic_polymers: Option<f64>,
}

#[derive(Deserialize)]
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
}

#[derive(Deserialize)]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum N2oEmissionFactorCalcMethod {
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
