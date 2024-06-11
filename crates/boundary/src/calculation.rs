use klick_domain::{self as domain, units::*};

use crate::{default_values, CalculationOutcome, EvaluationData, FormData};

// TODO:
// Handle these calculations as usecases in the domain layer.
pub fn calculate(form_data: FormData) -> CalculationOutcome {
    log::debug!("Calculate");
    let profile_input = default_values::profile(form_data.clone());
    let sensitivity_input = default_values::sensitivity(form_data.clone());
    let recommendation_input = default_values::recommendations(form_data);

    let custom_n2o_emission_factor = sensitivity_input
        .sensitivity_parameters
        .n2o_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to::<Factor>());

    let custom_ch4_chp_emission_factor = sensitivity_input
        .sensitivity_parameters
        .ch4_chp_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to::<Factor>());

    let (selected_ch4_chp_emission_factor_calc_method, _) = sensitivity_input
        .sensitivity_parameters
        .ch4_chp_emissions
        .try_into()
        .unwrap_or_else(|_| (None, None));

    let profile: Option<(_, _)> = profile_input.clone().try_into().ok(); // TODO: avoid clone
    let sensitivity: Option<(_, _)> = sensitivity_input.clone().try_into().ok(); // TODO: avoid clone
    let recommendation: Option<(_, _)> = recommendation_input.clone().try_into().ok(); // TODO: avoid clone

    let sensitivity_n2o_calculations = sensitivity.as_ref().map(|(values, _)| {
        log::debug!("Calculate all N2O emission factor scenarios");
        domain::calculate_all_n2o_emission_factor_scenarios(
            values,
            custom_n2o_emission_factor,
            selected_ch4_chp_emission_factor_calc_method,
            custom_ch4_chp_emission_factor,
        )
    });

    let sensitivity_ch4_chp_calculations = sensitivity.as_ref().map(|(values, _)| {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");
        domain::calculate_all_ch4_chp_emission_factor_scenarios(
            values,
            custom_ch4_chp_emission_factor,
        )
    });

    let profile_output =
        profile.map(|(values, methods)| domain::calculate_emissions(values, methods));
    let sensitivity_output =
        sensitivity.map(|(values, methods)| domain::calculate_emissions(values, methods));
    let recommendation_output =
        recommendation.map(|(values, methods)| domain::calculate_emissions(values, methods));

    CalculationOutcome {
        profile: EvaluationData {
            input: profile_input,
            output: profile_output,
        },
        sensitivity: EvaluationData {
            input: sensitivity_input,
            output: sensitivity_output,
        },
        recommendation: EvaluationData {
            input: recommendation_input,
            output: recommendation_output,
        },
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}
