use klick_domain::{self as domain, InputValueId as Id, Value as V};

use crate::{CalculationOutcome, FormData};

// TODO:
// Handle these calculations as usecases in the domain or application layer.
#[must_use]
pub fn calculate(input: FormData) -> CalculationOutcome {
    log::debug!("Calculate");

    let output = domain::calculate_emissions(&input)
        .map_err(|err| log::warn!("{err}"))
        .ok();

    log::debug!("Calculate all N2O emission factor scenarios");
    let sensitivity_n2o_calculations =
        domain::calculate_all_n2o_emission_factor_scenarios(&input).ok();

    let sensitivity_ch4_chp_calculations = {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");

        let sewage_gas_produced = domain::required_value(Id::SewageGasProduced, &input)
            .unwrap()
            .as_qubicmeters_unchecked();
        let methane_fraction = domain::required_value(Id::MethaneFraction, &input)
            .unwrap()
            .as_percent_unchecked();
        let custom_ch4_chp_emission_factor =
            domain::optional_value(Id::SensitivityCH4ChpCustomFactor, &input)
                .map(V::as_percent_unchecked);

        Some(domain::calculate_all_ch4_chp_emission_factor_scenarios(
            sewage_gas_produced,
            methane_fraction,
            custom_ch4_chp_emission_factor,
        ))
    };

    CalculationOutcome {
        input,
        output,
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}
