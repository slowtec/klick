use klick_boundary::CalculationOutcome;
use klick_domain::{
    units::{Percent, Ratio},
    CH4ChpEmissionFactorCalcMethod, CalculatedEmissionFactors, EmissionFactorCalculationMethods,
    N2oEmissionFactorCalcMethod,
};

use crate::{
    co2_equivalents_as_table, plant_profile_as_table, sensitivity_parameters_as_table,
    UnitFormatting,
};

pub fn calculation_outcome_as_csv(out: &CalculationOutcome) -> String {
    let unit = UnitFormatting::Text;

    let mut plant_profile_table =
        plant_profile_as_table(&out.recommendation.input.plant_profile, unit);
    let sensitivity_parameters_table = sensitivity_parameters_as_table(
        &out.recommendation.input.sensitivity_parameters,
        unit,
        &out.recommendation.output,
    );

    let co2_equivalents_table =
        co2_equivalents_as_table(&out.recommendation.output.co2_equivalents, unit);

    plant_profile_table
        .sections
        .extend(sensitivity_parameters_table.sections);

    plant_profile_table
        .sections
        .extend(co2_equivalents_table.sections);

    plant_profile_table
        .sections
        .into_iter()
        .map(|section| {
            let rows = section
                .rows
                .into_iter()
                .map(|(name, value, unit)| {
                    [
                        name,
                        &value
                            .map(|v|
                              // NOTE: this is required because 
                              // German values can contain ','
                              format!("\"{v}\""))
                            .unwrap_or_default(),
                        unit.unwrap_or_default(),
                    ]
                    .join(",")
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("## {}\n\n{rows}", section.title)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn emission_factor_calculation_methods_to_csv(
    methods: EmissionFactorCalculationMethods,
) -> String {
    [
        [
            "emission_factor_calculation_methods.n2o",
            n2o_emission_factor_calc_method_to_csv_name(&methods.n2o),
        ]
        .join(","),
        [
            "emission_factor_calculation_methods.ch4",
            match &methods.ch4 {
                Some(ch4) => ch4_chp_emission_factor_calc_method_to_csv_name(ch4),
                None => "Nicht festgelegt",
            },
        ]
        .join(","),
    ]
    .join("\n")
}

pub fn emission_factors_to_csv(factors: &CalculatedEmissionFactors) -> String {
    [
        format!(
            "{}, {}\n",
            "emission_factors.n2o",
            &f64::from(factors.n2o.convert_to::<Percent>()).to_string()
        ),
        format!(
            "{}, {}\n",
            "emission_factors.ch4",
            &f64::from(factors.ch4.convert_to::<Percent>()).to_string()
        ),
    ]
    .join("")
}

// TODO: use ValueLabel & ValueId
fn n2o_emission_factor_calc_method_to_csv_name(
    method: &N2oEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        N2oEmissionFactorCalcMethod::TuWien2016 => "TU Wien 2016",
        N2oEmissionFactorCalcMethod::Optimistic => "Optimistisch",
        N2oEmissionFactorCalcMethod::Pesimistic => "Pessimistisch",
        N2oEmissionFactorCalcMethod::Ipcc2019 => "PCC 2019",
        N2oEmissionFactorCalcMethod::Custom(_) => "Benutzerdefiniert",
    }
}

// TODO: use ValueLabel & ValueId
fn ch4_chp_emission_factor_calc_method_to_csv_name(
    method: &CH4ChpEmissionFactorCalcMethod,
) -> &'static str {
    match method {
        CH4ChpEmissionFactorCalcMethod::MicroGasTurbines => "Mikrogasturbinen",
        CH4ChpEmissionFactorCalcMethod::GasolineEngine => "Ottomotor",
        CH4ChpEmissionFactorCalcMethod::JetEngine => "Zündstrahlmotor",
        CH4ChpEmissionFactorCalcMethod::Custom(_) => "Benutzerdefiniert",
    }
}
