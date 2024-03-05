use derive_more::From;
use serde::Deserialize;
use time::OffsetDateTime;

pub use crate::v5::{
    AnnualAverage, CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, EnergyConsumption,
    N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario, OperatingMaterials,
    OptimizationScenario, ProjectId,
};

#[derive(Deserialize)]
pub struct Data {
    pub project: Project,
}

#[derive(Deserialize, From)]
#[serde(untagged)]
pub enum Project {
    Saved(SavedProject),
    Unsaved(ProjectData),
}

#[derive(Deserialize)]
pub struct ProjectData {
    pub title: Option<String>,
    pub plant_profile: PlantProfile,
    pub optimization_scenario: OptimizationScenario,
}

#[derive(Deserialize)]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: ProjectData,
}

#[derive(Deserialize)]
pub struct PlantProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plant_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub population_equivalent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wastewater: Option<f64>,

    pub influent_average: AnnualAverage,

    pub effluent_average: AnnualAverage,

    pub energy_consumption: EnergyConsumption,

    pub sewage_sludge_treatment: SewageSludgeTreatment,

    pub operating_materials: OperatingMaterials,
}

#[derive(Deserialize)]
pub struct SewageSludgeTreatment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_bags_are_open: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sludge_storage_containers_are_open: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sewage_sludge_for_disposal: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub digester_count: Option<u64>,
}
