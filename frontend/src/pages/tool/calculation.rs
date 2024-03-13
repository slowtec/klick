use klick_boundary::FormData;
use klick_domain::{
    self as domain, units::*, CH4ChpEmissionFactorCalcMethod, EmissionsCalculationOutcome,
    N2oEmissionFactorCalcMethod,
};

use super::default_values;

#[derive(Debug, Clone, PartialEq)]
pub struct CalculationOutcome {
    // a.k.a "Model One"
    pub profile: EmissionsCalculationOutcome,
    // a.k.a "Model Two"
    pub sensitivity: EmissionsCalculationOutcome,
    pub sensitivity_n2o_calculations:
        Vec<(N2oEmissionFactorCalcMethod, EmissionsCalculationOutcome)>,

    pub sensitivity_ch4_chp_calculations: Vec<(CH4ChpEmissionFactorCalcMethod, Tons, Factor)>,

    // a.k.a "Model Three"
    pub recommendation: EmissionsCalculationOutcome,
}

pub fn calculate(form_data: FormData) -> Option<CalculationOutcome> {
    log::debug!("Calculate");
    let profile = default_values::profile(form_data.clone());
    let sensitivity = default_values::sensitivity(form_data.clone());
    let recommendation = default_values::recommendations(form_data);

    let custom_n2o_emission_factor = sensitivity
        .sensitivity_parameters
        .n2o_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to());

    let custom_ch4_chp_emission_factor = sensitivity
        .sensitivity_parameters
        .ch4_chp_emissions
        .custom_emission_factor
        .map(|v| Percent::new(v).convert_to());

    let selected_ch4_chp_emission_factor_calc_method = sensitivity
        .sensitivity_parameters
        .ch4_chp_emissions
        .try_into()
        .ok();

    let profile: (_, _) = profile
        .try_into()
        //.map_err(|err| {
        //    log::warn!("{err}");
        //})
        .ok()?;
    let sensitivity: (_, _) = sensitivity.try_into().ok()?;
    let recommendation: (_, _) = recommendation.try_into().ok()?;

    let sensitivity_n2o_calculations = domain::calculate_all_n2o_emission_factor_scenarios(
        &sensitivity.0,
        custom_n2o_emission_factor,
        selected_ch4_chp_emission_factor_calc_method,
    );

    let sensitivity_ch4_chp_calculations = domain::calculate_all_ch4_chp_emission_factor_scenarios(
        &sensitivity.0,
        custom_ch4_chp_emission_factor,
    );

    let profile = domain::calculate_emissions(profile.0, profile.1);
    let sensitivity = domain::calculate_emissions(sensitivity.0, sensitivity.1);
    let recommendation = domain::calculate_emissions(recommendation.0, recommendation.1);

    let outcome = CalculationOutcome {
        profile,
        sensitivity,
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
        recommendation,
    };

    Some(outcome)
}
