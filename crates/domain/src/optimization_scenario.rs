use crate::Factor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptimizationScenario {
    pub n2o_emission_factor: N2oEmissionFactorCalcMethod,
    pub ch4_chp_emission_factor: Option<CH4ChpEmissionFactorCalcMethod>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum N2oEmissionFactorCalcMethod {
    TuWien2016,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    Custom(Factor),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CH4ChpEmissionFactorCalcMethod {
    MicroGasTurbines,
    GasolineEngine,
    JetEngine,
    Custom(Factor),
}
