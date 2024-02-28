use klick_domain::{
    units::{Percent, Ratio},
    CH4ChpEmissionFactorCalcMethod, EmissionFactorCalculationMethods, EmissionFactors,
    N2oEmissionFactorCalcMethod,
};

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

pub fn emission_factors_to_csv(factors: &EmissionFactors) -> String {
    [
        format!("{}, {}\n",
            "emission_factors.n2o",
            &f64::from(factors.n2o.convert_to::<Percent>()).to_string()),
        format!("{}, {}\n",
            "emission_factors.ch4",
            &f64::from(factors.ch4.convert_to::<Percent>()).to_string())
    ].join("")
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
