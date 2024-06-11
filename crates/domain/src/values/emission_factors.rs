use crate::units::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalculatedEmissionFactors {
    pub n2o: Factor,
    pub ch4: Factor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EmissionFactorCalculationMethods {
    pub n2o: N2oEmissionFactorCalcMethod,
    pub n2o_custom_factor: Option<Factor>,
    // TODO: rename to ch4_chp
    pub ch4: Option<Ch4ChpEmissionFactorCalcMethod>,
    pub ch4_custom_factor: Option<Factor>,
}
