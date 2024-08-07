use klick_domain as domain;

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
    let values = domain::EmissionInfluencingValues::try_from(profile_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();
    let profile: Option<(_, _)> = values.and_then(|v| methods.map(|m| (v, m)));

    // TODO: avoid clone
    let methods = domain::EmissionFactorCalculationMethods::try_from(sensitivity_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();
    // TODO: avoid clone
    let values = domain::EmissionInfluencingValues::try_from(sensitivity_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();
    let sensitivity: Option<(_, _)> = values.and_then(|v| methods.map(|m| (v, m)));

    // TODO: avoid clone
    let methods = domain::EmissionFactorCalculationMethods::try_from(recommendation_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();
    // TODO: avoid clone
    let values = domain::EmissionInfluencingValues::try_from(recommendation_input.clone())
        .map_err(|err| log::warn!("{err}"))
        .ok();
    let recommendation: Option<(_, _)> = values.and_then(|v| methods.map(|m| (v, m)));

    let profile_output =
        profile.map(|(values, methods)| domain::calculate_emissions(values, methods));
    let sensitivity_output =
        sensitivity.map(|(values, methods)| domain::calculate_emissions(values, methods));
    let recommendation_output =
        recommendation.map(|(values, methods)| domain::calculate_emissions(values, methods));

    //let sensitivity_n2o_calculations = sensitivity.as_ref().map(|(values, _)| {
    //    log::debug!("Calculate all N2O emission factor scenarios");
    //    domain::calculate_all_n2o_emission_factor_scenarios(
    //        values,
    //        custom_n2o_emission_factor,
    //        selected_ch4_chp_emission_factor_calc_method,
    //        custom_ch4_chp_emission_factor,
    //    )
    //});

    //let sensitivity_ch4_chp_calculations = sensitivity.as_ref().map(|(values, _)| {
    //    log::debug!("Calculate all CH4 CHP emission factor scenarios");
    //    domain::calculate_all_ch4_chp_emission_factor_scenarios(
    //        values,
    //        custom_ch4_chp_emission_factor,
    //    )
    //});

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
        sensitivity_n2o_calculations: None,     // FIXME
        sensitivity_ch4_chp_calculations: None, // FIXME
    }
}
