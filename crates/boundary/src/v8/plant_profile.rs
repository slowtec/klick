use serde::{Deserialize, Serialize};

pub use crate::v7::OperatingMaterials;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct PlantProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_equivalent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wastewater: Option<f64>,

    pub influent_average: AnnualAverageInfluent,
    pub effluent_average: AnnualAverageEffluent,
    pub energy_consumption: EnergyConsumption,
    pub sewage_sludge_treatment: SewageSludgeTreatment,
    pub side_stream_treatment: SideStreamTreatment,
    pub operating_materials: OperatingMaterials,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverageInfluent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_organic_carbohydrates: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverageEffluent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nitrogen: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct EnergyConsumption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_power_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_site_power_generation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_electricity_mix: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_supply: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_of_biogas: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_oil: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_gas_produced: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub methane_fraction: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SewageSludgeTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digester_count: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_bags_are_closed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_storage_containers_are_closed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_sludge_for_disposal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SideStreamTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nitrogen: Option<f64>,
}