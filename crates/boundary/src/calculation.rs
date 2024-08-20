use std::collections::HashMap;

use klick_domain::{
    self as domain,
    input_value::{optional, required},
    Id, InputValueId as In,
};

use crate::CalculationOutcome;

// TODO:
// Handle these calculations as usecases in the domain or application layer.
#[must_use]
pub fn calculate(
    input: &HashMap<Id, domain::Value>,
    custom_edges: Option<&[(Id, Id)]>,
) -> CalculationOutcome {
    log::debug!("Calculate");

    let output = domain::calculate(input, custom_edges)
        .map_err(|err| log::warn!("{err}"))
        .ok();

    log::debug!("Calculate all N2O emission factor scenarios");
    let sensitivity_n2o_calculations =
        domain::calculate_all_n2o_emission_factor_scenarios(&input, None).ok();

    let sensitivity_ch4_chp_calculations = {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");

        let sewage_gas_produced = required!(In::SewageGasProduced, &input).unwrap();
        let methane_fraction = required!(In::MethaneFraction, &input).unwrap();
        let custom_ch4_chp_emission_factor = optional!(In::SensitivityCH4ChpCustomFactor, &input);

        Some(domain::calculate_all_ch4_chp_emission_factor_scenarios(
            sewage_gas_produced,
            methane_fraction,
            custom_ch4_chp_emission_factor,
        ))
    };

    CalculationOutcome {
        input: input.clone(),
        output,
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}
