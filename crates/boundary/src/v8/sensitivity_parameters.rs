use serde::{Deserialize, Serialize};

pub use crate::v7::{CH4ChpEmissionFactorCalcMethod, N2oEmissionFactorCalcMethod};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct SensitivityParameters {
    pub n2o_emissions: N2OEmissionsSensitivity,
    pub ch4_chp_emissions: CH4ChpEmissionsSensitivity,
    pub ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity,
    pub co2_fossil_emissions: FossilEmissonsSensitivity,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "extra-derive",
    derive(Default, Debug, Copy, Clone, PartialEq)
)]
pub struct N2OEmissionsSensitivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_method: Option<N2oEmissionFactorCalcMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emission_factor: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_stream_emission_factor: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(
    feature = "extra-derive",
    derive(Default, Debug, Copy, Clone, PartialEq)
)]
pub struct CH4ChpEmissionsSensitivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculation_method: Option<CH4ChpEmissionFactorCalcMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emission_factor: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct SewageSludgeTreatmentEmissionsSensitivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_sludge_bags: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor_sludge_storage_containers: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Default, Debug, Clone, PartialEq))]
pub struct FossilEmissonsSensitivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emission_factor: Option<f64>,
}
