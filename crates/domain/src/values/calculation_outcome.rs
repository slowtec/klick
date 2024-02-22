use crate::{
    CH4ChpEmissionFactorCalcMethod, CO2Equivalents, EmissionFactorCalculationMethods,
    EmissionFactors, N2oEmissionFactorCalcMethod,
};

#[derive(Debug, Clone)]
pub struct EmissionsCalculationOutcome {
    pub co2_equivalents: CO2Equivalents,
    pub emission_factors: EmissionFactors,
    pub calculation_methods: EmissionFactorCalculationMethods,
}

#[derive(Debug, Clone)]
pub struct CalculatedScenarios {
    pub n2o_emission_factor_scenarios: Vec<N2oEmissionFactorScenario>,
    pub ch4_emission_factor_scenarios: Vec<Ch4EmissionFactorScenario>,
    pub ch4_sludge_treatment_scenarios: Vec<Ch4SludgeTreatmentScenario>,
}

#[derive(Debug, Clone)]
pub struct N2oEmissionFactorScenario {
    pub calculation_method: N2oEmissionFactorCalcMethod,
    pub outcome: EmissionsCalculationOutcome,
}

#[derive(Debug, Clone)]
pub struct Ch4EmissionFactorScenario {
    pub calculation_method: CH4ChpEmissionFactorCalcMethod,
    pub outcome: EmissionsCalculationOutcome,
}

#[derive(Debug, Clone)]
pub struct Ch4SludgeTreatmentScenario {
    pub closed_sludge_bags: bool,
    pub closed_sludge_storage_containers: bool,
    pub outcome: EmissionsCalculationOutcome,
}
