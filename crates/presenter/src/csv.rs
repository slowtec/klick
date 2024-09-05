use std::collections::HashMap;

use klick_boundary::CalculationOutcome;
use klick_domain::{
    units::{Factor, Percent, RatioExt},
    OutputValueId as Out,
};

use crate::{
    co2_equivalents_as_table, plant_profile_as_table, sensitivity_parameters_as_table, Formatting,
    Lng,
};

#[must_use]
pub fn calculation_outcome_as_csv(out: &CalculationOutcome, lang: Lng) -> String {
    let unit = Formatting::Text;

    let mut plant_profile_table = plant_profile_as_table(&out.input, unit, lang);

    let sensitivity_parameters_table = sensitivity_parameters_as_table(&out.input, unit);

    let co2_equivalents_table = out
        .output
        .clone()
        .zip(out.graph.clone())
        .map(|out| co2_equivalents_as_table(&out, unit))
        .unwrap_or_default();

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
                        value
                            .map(|v|
                              // NOTE: this is required because 
                              // German values can contain ','
                              format!("\"{v}\""))
                            .unwrap_or_default(),
                        unit.unwrap_or_default().to_string(),
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

#[must_use]
pub fn emission_factors_to_csv(factors: &HashMap<Out, Factor>) -> String {
    [
        format!(
            "{}, {}\n",
            "emission_factors.n2o",
            &f64::from(
                factors
                    .get(&Out::N2oCalculatedEmissionFactor)
                    .unwrap()
                    .convert_to::<Percent>()
            )
            .to_string()
        ),
        format!(
            "{}, {}\n",
            "emission_factors.ch4",
            &f64::from(
                factors
                    .get(&Out::Ch4ChpCalculatedEmissionFactor)
                    .unwrap()
                    .convert_to::<Percent>()
            )
            .to_string()
        ),
    ]
    .join("")
}
