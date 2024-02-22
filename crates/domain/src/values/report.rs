use crate::{
    CH4ChpEmissionFactorCalcMethod, CO2Equivalents, CalculatedScenarios, EmissionInfluencingValues,
    N2oEmissionFactorCalcMethod,
};

pub struct Report {
    pub initial_situation: EmissionInfluencingValues,
    pub scenarios: CalculatedScenarios,
    pub selection: ScenarioSelection,
    pub total_difference: CO2Equivalents,
}

pub struct ScenarioSelection {
    pub n2o_calculation_method: N2oEmissionFactorCalcMethod,
    pub ch4_calculation_method: CH4ChpEmissionFactorCalcMethod,
    pub closed_sludge_bags: bool,
    pub closed_sludge_storage_containers: bool,
}
