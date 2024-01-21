use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use derive_more::From;

pub use crate::v4::{
    AnnualAverage, CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, EnergyConsumption,
    InputData as PlantProfile, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario,
    OperatingMaterials, Scenario as OptimizationScenario, SewageSludgeTreatment,
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
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: ProjectData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct ProjectData {
    pub title: Option<String>,
    pub plant_profile: PlantProfile,
    pub optimization_scenario: OptimizationScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Copy, Clone, PartialEq, From))]
pub struct ProjectId(pub Uuid);
