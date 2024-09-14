use klick_domain::units::*;

use crate::calculate_ch4_chp;

const CH4_CHP_CALC_METHODS: [Ch4ChpEmissionFactorCalcMethod; 3] = [
    Ch4ChpEmissionFactorCalcMethod::MicroGasTurbines,
    Ch4ChpEmissionFactorCalcMethod::GasolineEngine,
    Ch4ChpEmissionFactorCalcMethod::JetEngine,
];

#[must_use]
pub fn calculate_all_ch4_chp_emission_factor_scenarios(
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
    custom_factor: Option<Percent>,
) -> Vec<(Ch4ChpEmissionFactorCalcMethod, Tons, Factor)> {
    let mut results = CH4_CHP_CALC_METHODS
        .into_iter()
        .map(|method| {
            let (result, factor) =
                calculate_ch4_chp(Some(method), None, sewage_gas_produced, methane_fraction);
            (method, result, factor)
        })
        .collect();

    let Some(factor) = custom_factor else {
        return results;
    };

    // Custom
    let method = Ch4ChpEmissionFactorCalcMethod::Custom;
    let (result, factor) = calculate_ch4_chp(
        Some(method),
        Some(factor),
        sewage_gas_produced,
        methane_fraction,
    );
    results.push((method, result, factor));

    results
}
