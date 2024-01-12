use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub use crate::v3::{
    AnnualAverage, CH4ChpEmissionFactorCalcMethod, CH4ChpEmissionFactorScenario, EnergyConsumption,
    InputData, OperatingMaterials, SewageSludgeTreatment,
};

#[derive(Deserialize)]
pub struct Import {
    pub input: InputData,
    pub scenario: Scenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
    pub ch4_chp_emission_factor: Option<CH4ChpEmissionFactorScenario>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq, Eq))]
pub enum N2oEmissionFactorCalcMethod {
    #[cfg_attr(feature = "extra-derive", default)]
    TuWien2016,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
