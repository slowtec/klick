use crate::units::Factor;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalculatedEmissionFactors {
    pub n2o: Factor,
    pub ch4: Factor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmissionFactorCalculationMethods {
    pub n2o: N2oEmissionFactorCalcMethod,
    // TODO: rename to ch4_chp
    pub ch4: Option<CH4ChpEmissionFactorCalcMethod>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum N2oEmissionFactorCalcMethod {
    #[default]
    TuWien2016,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    Custom(Factor),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CH4ChpEmissionFactorCalcMethod {
    #[default]
    GasolineEngine,
    MicroGasTurbines,
    JetEngine,
    Custom(Factor),
}
