use derive_more::From;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub use crate::v7::{
    CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, N2oEmissionFactorCalcMethod,
    N2oEmissionFactorScenario, OperatingMaterials, OptimizationScenario, ProjectId,
};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
pub struct Data {
    pub project: Project,
}

#[derive(Serialize, Deserialize, From)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
#[serde(untagged)]
pub enum Project {
    Saved(SavedProject),
    Unsaved(ProjectData),
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct ProjectData {
    pub title: Option<String>,
    pub plant_profile: PlantProfile,
    pub optimization_scenario: OptimizationScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: ProjectData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
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

    pub emission_factors: CustomEmissionFactors,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct CustomEmissionFactors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n2o_side_stream: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co2_fossil: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SideStreamTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_nitrogen: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_stream_cover_is_open: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct SewageSludgeTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_bags_are_open: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_bags_are_open_recommendation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sludge_bags_factor: Option<f64>, // FIXME move to EmissionFactors

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_storage_containers_are_open: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_storage_containers_are_open_recommendation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sludge_storage_containers_factor: Option<f64>, // FIXME move to EmissionFactors

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_sludge_for_disposal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub digester_count: Option<u64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct EnergyConsumption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_gas_produced: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub methane_fraction: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_supply: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_of_biogas: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_power_consumption: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_site_power_generation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_electricity_mix: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heating_oil: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverageInfluent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_organic_carbohydrates: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct AnnualAverageEffluent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nitrogen: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemical_oxygen_demand: Option<f64>,
}
