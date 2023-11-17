use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_values: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub waste_water: Option<f64>,

    pub inflow_averages: AnnualAverages,

    pub effluent_averages: AnnualAverages,

    pub energy_consumption: EnergyConsumption,

    pub sewage_sludge_treatment: SewageSludgeTreatment,

    pub operating_materials: OperatingMaterials,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_n2o_emission_factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualAverages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phosphorus: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConsumption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_gas_produced: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub methane_level: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_supply: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_of_biogas: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_power_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_house_power_generation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_electricity_mix: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
