use serde::{Deserialize, Serialize};

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
    pub plant_name: Option<String>,
    pub population_equivalent: Option<f64>,
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

#[derive(Deserialize)]
pub struct CH4ChpEmissionFactorScenario {
    pub calculation_method: CH4ChpEmissionFactorCalcMethod,
    pub custom_factor: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CH4ChpEmissionFactorCalcMethod {
    MicroGasTurbines,
    GasolineEngine,
    JetEngine,
    CustomFactor,
}
