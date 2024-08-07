use std::collections::HashMap;

use crate::{optional_value, required_value, units::*, InputValueId as Id, Value as V};

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

impl TryFrom<HashMap<Id, Value>> for EmissionFactorCalculationMethods {
    type Error = anyhow::Error;
    fn try_from(mut from: HashMap<Id, Value>) -> Result<Self, Self::Error> {
        let n2o = required_value(Id::SensitivityN2OCalculationMethod, &mut from)?
            .as_n2o_emission_factor_calc_method_unchecked();
        let ch4 = optional_value(Id::SensitivityCH4ChpCalculationMethod, &mut from)
            .map(V::as_ch4_chp_emission_factor_calc_method_unchecked);

        // FIXME: the conversion should not be necessary
        let n2o_custom_factor = optional_value(Id::SensitivityN2OCustomFactor, &mut from)
            .map(V::as_percent_unchecked)
            .map(|f| f.convert_to::<Factor>());

        // FIXME: the conversion should not be necessary
        let ch4_custom_factor = optional_value(Id::SensitivityCH4ChpCustomFactor, &mut from)
            .map(V::as_percent_unchecked)
            .map(|f| f.convert_to::<Factor>());

        Ok(Self {
            n2o,
            n2o_custom_factor,
            ch4,
            ch4_custom_factor,
        })
    }
}
