use klick_domain::{
    self as domain,
    units::{Factor, RatioExt},
    InputValueId as Id, Value as V,
};

use crate::{CalculationOutcome, EvaluationData, FormData};

// TODO:
// Handle these calculations as usecases in the domain or application layer.
#[must_use]
pub fn calculate(form_data: FormData) -> CalculationOutcome {
    log::debug!("Calculate");
    let profile_input = form_data.clone();
    let sensitivity_input = form_data.clone();
    let recommendation_input = form_data;

    // TODO: avoid clone
    let methods = domain::EmissionFactorCalculationMethods::try_from(profile_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();

    // TODO: avoid clone
    let values = profile_input.clone();
    let profile: Option<(_, _)> = methods.map(|m| (values, m));

    // TODO: avoid clone
    let methods = domain::EmissionFactorCalculationMethods::try_from(sensitivity_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();

    // TODO: avoid clone
    let values = sensitivity_input.clone();
    let sensitivity: Option<(_, _)> = methods.map(|m| (values, m));

    // TODO: avoid clone
    let methods = domain::EmissionFactorCalculationMethods::try_from(recommendation_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();

    // TODO: avoid clone
    let values = recommendation_input.clone();
    let recommendation: Option<(_, _)> = methods.map(|m| (values, m));

    let profile_output = profile.and_then(|(values, methods)| {
        domain::calculate_emissions(&values, methods)
            .map_err(|err| log::warn!("{err}"))
            .ok()
    });

    // TODO: avoid clone
    let sensitivity_output = sensitivity.clone().and_then(|(values, methods)| {
        domain::calculate_emissions(&values, methods)
            .map_err(|err| log::warn!("{err}"))
            .ok()
    });

    let recommendation_output = recommendation.and_then(|(values, methods)| {
        domain::calculate_emissions(&values, methods)
            .map_err(|err| log::warn!("{err}"))
            .ok()
    });

    // TODO: avoid clone
    let sensitivity_n2o_calculations = sensitivity.clone().and_then(|(mut values, _)| {
        log::debug!("Calculate all N2O emission factor scenarios");

        let custom_n2o_emission_factor =
            domain::optional_value(Id::SensitivityN2OCustomFactor, &mut values)
                .map(V::as_percent_unchecked)
                .map(|v| v.convert_to::<Factor>());
        let custom_ch4_chp_emission_factor =
            domain::optional_value(Id::SensitivityCH4ChpCustomFactor, &mut values)
                .map(V::as_percent_unchecked)
                .map(|v| v.convert_to::<Factor>());

        let selected_ch4_chp_emission_factor_calc_method =
            domain::optional_value(Id::SensitivityCH4ChpCalculationMethod, &mut values)
                .map(V::as_ch4_chp_emission_factor_calc_method_unchecked);

        domain::calculate_all_n2o_emission_factor_scenarios(
            &values,
            custom_n2o_emission_factor,
            selected_ch4_chp_emission_factor_calc_method,
            custom_ch4_chp_emission_factor,
        )
        .ok()
    });

    // TODO: avoid clone
    let sensitivity_ch4_chp_calculations = sensitivity.clone().map(|(mut values, _)| {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");

        let sewage_gas_produced = domain::required_value(Id::SewageGasProduced, &mut values)
            .unwrap()
            .as_qubicmeters_unchecked();
        let methane_fraction = domain::required_value(Id::MethaneFraction, &mut values)
            .unwrap()
            .as_percent_unchecked();
        let custom_ch4_chp_emission_factor =
            domain::optional_value(Id::SensitivityCH4ChpCustomFactor, &mut values)
                .map(V::as_percent_unchecked);

        domain::calculate_all_ch4_chp_emission_factor_scenarios(
            sewage_gas_produced,
            methane_fraction,
            custom_ch4_chp_emission_factor,
        )
    });

    CalculationOutcome {
        profile: EvaluationData {
            input: profile_input.into(),
            output: profile_output,
        },
        sensitivity: EvaluationData {
            input: sensitivity_input.into(),
            output: sensitivity_output,
        },
        recommendation: EvaluationData {
            input: recommendation_input.into(),
            output: recommendation_output,
        },
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}
