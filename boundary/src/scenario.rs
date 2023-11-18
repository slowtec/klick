use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct Scenario {
    pub n2o_emission_factor: N2oEmissionFactorScenario,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize, EnumIter)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(
    feature = "extra-derive",
    derive(Debug, Default, Clone, Copy, PartialEq, Eq)
)]
pub enum N2oEmissionFactorCalcMethod {
    #[cfg_attr(feature = "extra-derive", default)]
    ExtrapolatedParravicini,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    CustomFactor,
}
