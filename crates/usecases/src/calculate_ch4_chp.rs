use klick_domain::{constants::*, units::*};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn calculate_ch4_chp(
    calculation_method: Option<Ch4ChpEmissionFactorCalcMethod>,
    custom_factor: Option<Percent>,
    sewage_gas_produced: Qubicmeters,
    methane_fraction: Percent,
) -> (Tons, Factor) {
    let ch4_emission_factor = match calculation_method {
        Some(Ch4ChpEmissionFactorCalcMethod::MicroGasTurbines) => Factor::new(0.01),
        Some(Ch4ChpEmissionFactorCalcMethod::GasolineEngine) | None => Factor::new(0.015), // FIXME None is a hack and it seems to not use the default value from units.rs
        Some(Ch4ChpEmissionFactorCalcMethod::JetEngine) => Factor::new(0.025),
        Some(Ch4ChpEmissionFactorCalcMethod::Custom) => {
            custom_factor.expect("custom CH4 EF").into()
        }
    };

    let volume = sewage_gas_produced * methane_fraction * ch4_emission_factor;
    let mass = volume * CONVERSION_FACTOR_CH4_M3_TO_KG;
    let ch4_chp = mass.convert_to::<Tons>();

    (ch4_chp * GWP_CH4, ch4_emission_factor)
}
