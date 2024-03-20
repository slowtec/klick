use klick_boundary::{self as boundary, FormData};
use klick_domain::{
    self as domain, units::*, CH4ChpEmissionFactorCalcMethod, EmissionsCalculationOutcome,
    N2oEmissionFactorCalcMethod,
};

use super::default_values;

#[derive(Debug, Clone, PartialEq)]
pub struct CalculationOutcome {
    // a.k.a "Model One"
    pub profile: CalculationInputOutput,

    // a.k.a "Model Two"
    pub sensitivity: CalculationInputOutput,

    // Used to create bar chart input
    pub sensitivity_n2o_calculations:
        Vec<(N2oEmissionFactorCalcMethod, EmissionsCalculationOutcome)>,

    // Used to create bar chart input
    pub sensitivity_ch4_chp_calculations: Vec<(CH4ChpEmissionFactorCalcMethod, Tons, Factor)>,

    // a.k.a "Model Three"
    pub recommendation: CalculationInputOutput,
}

// TODO: Is there a better name for this struct?
#[derive(Debug, Clone, PartialEq)]
pub struct CalculationInputOutput {
    pub input: boundary::FormData,
    pub output: domain::EmissionsCalculationOutcome,
}

pub fn calculate(form_data: FormData) -> Option<CalculationOutcome> {
    log::debug!("Calculate");
    let profile_input = default_values::profile(form_data.clone());
    let sensitivity_input = default_values::sensitivity(form_data.clone());
    let recommendation_input = default_values::recommendations(form_data);

    let custom_n2o_emission_factor = sensitivity_input
        .sensitivity_parameters
        .n2o_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to());

    let custom_ch4_chp_emission_factor = sensitivity_input
        .sensitivity_parameters
        .ch4_chp_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to());

    let selected_ch4_chp_emission_factor_calc_method = sensitivity_input
        .sensitivity_parameters
        .ch4_chp_emissions
        .try_into()
        .ok();

    let profile: (_, _) = profile_input.clone().try_into().ok()?; // TODO: avoid clone
    let sensitivity: (_, _) = sensitivity_input.clone().try_into().ok()?; // TODO: avoid clone
    let recommendation: (_, _) = recommendation_input.clone().try_into().ok()?; // TODO: avoid clone

    let sensitivity_n2o_calculations = domain::calculate_all_n2o_emission_factor_scenarios(
        &sensitivity.0,
        custom_n2o_emission_factor,
        selected_ch4_chp_emission_factor_calc_method,
    );

    let sensitivity_ch4_chp_calculations = domain::calculate_all_ch4_chp_emission_factor_scenarios(
        &sensitivity.0,
        custom_ch4_chp_emission_factor,
    );

    let profile_output = domain::calculate_emissions(profile.0, profile.1);
    let sensitivity_output = domain::calculate_emissions(sensitivity.0, sensitivity.1);
    let recommendation_output = domain::calculate_emissions(recommendation.0, recommendation.1);

    let outcome = CalculationOutcome {
        profile: CalculationInputOutput {
            input: profile_input,
            output: profile_output,
        },
        sensitivity: CalculationInputOutput {
            input: sensitivity_input,
            output: sensitivity_output,
        },
        recommendation: CalculationInputOutput {
            input: recommendation_input,
            output: recommendation_output,
        },
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    };

    Some(outcome)
}
