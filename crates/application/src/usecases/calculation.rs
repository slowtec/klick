use klick_domain::{
    calculate_emissions, units::Factor, CH4ChpEmissionFactorCalcMethod, CO2Equivalents,
    EmissionFactorCalculationMethods, EmissionFactors, EmissionInfluencingValues,
    N2oEmissionFactorCalcMethod,
};

pub fn calculate_all_n2o_emission_factor_scenarios(
    values: &EmissionInfluencingValues,
    custom_factor: Option<Factor>,
    ch4_chp_calc_method: Option<CH4ChpEmissionFactorCalcMethod>,
) -> Vec<(N2oEmissionFactorCalcMethod, CO2Equivalents, EmissionFactors)> {
    let ch4 = ch4_chp_calc_method;

    // TuWien2016
    let n2o = N2oEmissionFactorCalcMethod::TuWien2016;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let (emissions, factors, _) = calculate_emissions(values, methods);
    let tuwien2016_result = (n2o, emissions, factors);

    // Optimistic
    let n2o = N2oEmissionFactorCalcMethod::Optimistic;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let (emissions, factors, _) = calculate_emissions(values, methods);
    let optimistc_result = (n2o, emissions, factors);

    // Pesimistic
    let n2o = N2oEmissionFactorCalcMethod::Pesimistic;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let (emissions, factors, _) = calculate_emissions(values, methods);
    let pesimistic_result = (n2o, emissions, factors);

    // Ipcc2019
    let n2o = N2oEmissionFactorCalcMethod::Ipcc2019;
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let (emissions, factors, _) = calculate_emissions(values, methods);
    let ipcc2019_result = (n2o, emissions, factors);

    let mut results = vec![
        tuwien2016_result,
        optimistc_result,
        pesimistic_result,
        ipcc2019_result,
    ];

    // Custom
    let Some(factor) = custom_factor else {
        return results;
    };

    let n2o = N2oEmissionFactorCalcMethod::Custom(factor);
    let methods = EmissionFactorCalculationMethods { n2o, ch4 };
    let (emissions, factors, _) = calculate_emissions(values, methods);
    let custom_result = (n2o, emissions, factors);
    results.push(custom_result);

    results
}
