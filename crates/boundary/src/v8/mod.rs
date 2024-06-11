use derive_more::From;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use klick_domain as domain;

pub use crate::v7::{
    CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, N2oEmissionFactorScenario,
    ProjectId,
};

mod optimization_scenario;
mod plant_profile;
mod sensitivity_parameters;

pub use self::{optimization_scenario::*, plant_profile::*, sensitivity_parameters::*};

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

// NOTE:
// In the future, we want to use a HashMap,
// which is why we are first implementing manual access via variable IDs.
impl FormData {
    pub fn get(&self, id: &domain::InputValueId) -> Option<domain::Value> {
        use domain::{units::*, InputValueId as Id, Value as V};

        match id {
            Id::PlantName => self.plant_profile.plant_name.clone().map(V::Text),
            Id::PopulationEquivalent => self
                .plant_profile
                .population_equivalent
                .map(|v| v as u64)
                .map(Into::into),
            Id::Wastewater => self
                .plant_profile
                .wastewater
                .map(Qubicmeters::new)
                .map(Into::into),
            _ => {
                panic!("TODO: implement read access of {id:?} via FormData::get method");
            }
        }
    }

    pub fn set(&mut self, id: domain::InputValueId, value: Option<domain::Value>) {
        use domain::{units::*, InputValueId as Id, Value as V};
        match id {
            Id::PlantName => {
                self.plant_profile.plant_name = value.map(V::expect_text);
            }
            Id::PopulationEquivalent => {
                self.plant_profile.population_equivalent =
                    value.map(V::expect_int).map(|v| v as f64);
            }
            Id::Wastewater => {
                self.plant_profile.wastewater = value
                    .map(V::expect_quantity)
                    .map(Quantity::expect_volume)
                    .map(Volume::expect_qubicmeters)
                    .map(Into::into);
            }
            _ => {
                panic!("TODO: implement write access of {id:?} via FormData::set method");
            }
        }
    }
}
