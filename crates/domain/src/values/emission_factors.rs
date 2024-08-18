use std::collections::HashMap;

use klick_value::{
    extract_optional, extract_required,
    specs::{InputValueId as Id, MissingInputValueIdError},
    units::*,
};

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
        let n2o = extract_required!(Id::SensitivityN2OCalculationMethod, &mut from)?;
        let ch4 = extract_optional!(Id::SensitivityCH4ChpCalculationMethod, &mut from);

        // FIXME: the conversion should not be necessary
        let n2o_custom_factor = extract_optional!(Id::SensitivityN2OCustomFactor, &mut from)
            .map(|f| f.convert_to::<Factor>());

        // FIXME: the conversion should not be necessary
        let ch4_custom_factor = extract_optional!(Id::SensitivityCH4ChpCustomFactor, &mut from)
            .map(|f| f.convert_to::<Factor>());

        Ok(Self {
            n2o,
            n2o_custom_factor,
            ch4,
            ch4_custom_factor,
        })
    }
}
