use serde::{Deserialize, Serialize};
use strum::EnumIter;

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

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phosphorus: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SewageSludgeTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sludge_bags: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_sludge_storage_containers: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_sludge_for_disposal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct OperatingMaterials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecl3: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub feclso4: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caoh2: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthetic_polymers: Option<f64>,
}

#[derive(Deserialize)]
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Default, Clone, Copy, PartialEq, Eq)
)]
pub enum N2oEmissionFactorCalcMethod {
    #[cfg_attr(feature = "extra-derive", default)]
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
