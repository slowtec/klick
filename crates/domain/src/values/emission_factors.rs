use crate::units::Factor;

#[derive(Debug, Clone, Copy)]
pub struct EmissionFactors {
    pub n2o: Factor,
    pub ch4: Factor,
}

#[derive(Debug, Clone, Copy)]
pub struct EmissionFactorCalculationMethods {
    pub n2o: N2oEmissionFactorCalcMethod,
    // TODO: rename to ch3_chp
    pub ch4: Option<CH4ChpEmissionFactorCalcMethod>,
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
