use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub use crate::v2::{
    AnnualAverage, EnergyConsumption, N2oEmissionFactorCalcMethod, N2oEmissionFactorScenario,
    OperatingMaterials, SewageSludgeTreatment,
};

#[derive(Deserialize)]
pub(crate) struct Import {
    pub(crate) input: InputData,
    pub(crate) scenario: Scenario,
}

#[derive(Deserialize)]
pub struct InputData {
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
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
    pub ch4_chp_emission_factor: Option<CH4ChpEmissionFactorScenario>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
pub struct CH4ChpEmissionFactorScenario {
    pub calculation_method: CH4ChpEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, Copy, PartialEq, Eq))]
pub enum CH4ChpEmissionFactorCalcMethod {
    MicroGasTurbines,
    GasolineEngine,
    JetEngine,
    CustomFactor,
}
