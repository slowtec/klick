use derive_more::From;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

mod optimization_scenario;
mod plant_profile;
mod sensitivity_parameters;

pub use self::{optimization_scenario::*, plant_profile::*, sensitivity_parameters::*};
pub use crate::v7::{
    CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, N2oEmissionFactorScenario,
    ProjectId,
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
    Unsaved(FormData),
}

impl Project {
    #[must_use]
    pub const fn form_data(&self) -> &FormData {
        match self {
            Self::Saved(SavedProject { data, .. }) | Self::Unsaved(data) => data,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct SavedProject {
    pub id: ProjectId,
    pub created_at: OffsetDateTime,
    pub modified_at: Option<OffsetDateTime>,
    #[serde(flatten)]
    pub data: FormData,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct FormData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_title: Option<String>,
    // First page in the tool frontend
    pub plant_profile: PlantProfile,
    // Second page in the tool frontend
    pub sensitivity_parameters: SensitivityParameters,
    // Third page in the tool frontend
    pub optimization_scenario: OptimizationScenario,
}
