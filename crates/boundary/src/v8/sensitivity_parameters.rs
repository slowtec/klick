use serde::Deserialize;

pub use crate::v7::{CH4ChpEmissionFactorCalcMethod, N2oEmissionFactorCalcMethod};

#[derive(Deserialize, Default)]
pub struct SensitivityParameters {
    pub n2o_emissions: N2OEmissionsSensitivity,
    pub ch4_chp_emissions: CH4ChpEmissionsSensitivity,
    pub ch4_sewage_sludge_emissions: SewageSludgeTreatmentEmissionsSensitivity,
    pub co2_fossil_emissions: FossilEmissonsSensitivity,
}

#[derive(Deserialize, Default)]
pub struct N2OEmissionsSensitivity {
    pub calculation_method: Option<N2oEmissionFactorCalcMethod>,
    pub custom_emission_factor: Option<f64>,
    pub side_stream_emission_factor: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct CH4ChpEmissionsSensitivity {
    pub calculation_method: Option<CH4ChpEmissionFactorCalcMethod>,
    pub custom_emission_factor: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct SewageSludgeTreatmentEmissionsSensitivity {
    pub emission_factor_sludge_bags: Option<f64>,
    pub emission_factor_sludge_storage_containers: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct FossilEmissonsSensitivity {
    pub emission_factor: Option<f64>,
}
