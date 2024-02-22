use crate::units::Factor;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct EmissionFactors {
    pub n2o: Factor,
    pub ch4: Factor,
}

// FIXME: move to presenter
impl EmissionFactors {
    pub fn to_csv(&self) -> String {
        let mut output: String = String::new();
        output += &format!("\nemission_factors.n2o, {}", f64::from(self.n2o) * 100.0);
        output += &format!("\nemission_factors.ch4, {}", f64::from(self.ch4) * 100.0);
        // output += &format!("\nemission_factors.n2o, {}", f64::from(self.n2o.convert_into::<Percent>()));
        // output += &format!("\nemission_factors.ch4, {}", f64::from(self.ch4.convert_into::<Percent>()));
        output
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EmissionFactorCalculationMethods {
    pub n2o: N2oEmissionFactorCalcMethod,
    // TODO: rename to ch3_chp
    pub ch4: Option<CH4ChpEmissionFactorCalcMethod>,
}

// FIXME: move to presenter
impl EmissionFactorCalculationMethods {
    pub fn to_csv(&self) -> String {
        let mut output: String = String::new();
        output += &format!("\nemission_factor_calculation_methods.n2o, {}", self.n2o);
        match &self.ch4 {
            Some(ch4) => output += &format!("\nemission_factor_calculation_methods.ch4, {}", ch4),
            None => {
                output += &format!(
                    "\nemission_factor_calculation_methods.ch4, {}",
                    "Nicht festgelegt"
                )
            }
        }
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum N2oEmissionFactorCalcMethod {
    TuWien2016,
    Optimistic,
    Pesimistic,
    Ipcc2019,
    Custom(Factor),
}

// FIXME: move to presenter
impl fmt::Display for N2oEmissionFactorCalcMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            N2oEmissionFactorCalcMethod::TuWien2016 => write!(f, "TU Wien 2016"),
            N2oEmissionFactorCalcMethod::Optimistic => write!(f, "Optimistisch"),
            N2oEmissionFactorCalcMethod::Pesimistic => write!(f, "Pessimistisch"),
            N2oEmissionFactorCalcMethod::Ipcc2019 => write!(f, "PCC 2019"),
            N2oEmissionFactorCalcMethod::Custom(_) => write!(f, "Benutzerdefiniert"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CH4ChpEmissionFactorCalcMethod {
    MicroGasTurbines,
    GasolineEngine,
    JetEngine,
    Custom(Factor),
}

// FIXME: move to presenter
impl fmt::Display for CH4ChpEmissionFactorCalcMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            CH4ChpEmissionFactorCalcMethod::MicroGasTurbines => write!(f, "Mikrogasturbinen"),
            CH4ChpEmissionFactorCalcMethod::GasolineEngine => write!(f, "Ottomotor"),
            CH4ChpEmissionFactorCalcMethod::JetEngine => write!(f, "Zündstrahlmotor"),
            CH4ChpEmissionFactorCalcMethod::Custom(_) => write!(f, "Benutzerdefiniert"),
        }
    }
}
