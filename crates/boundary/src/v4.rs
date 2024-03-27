#[cfg(feature = "extra-derive")]
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

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
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
    pub ch4_chp_emission_factor: Option<CH4ChpEmissionFactorScenario>,
}

#[derive(Serialize, Deserialize)]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)
)]
pub enum N2oEmissionFactorCalcMethod {
    TuWien2016,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
