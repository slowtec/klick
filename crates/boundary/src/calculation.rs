use std::collections::HashMap;

use klick_domain::{
    self as domain,
    input_value::{optional, required},
    units::Tons,
    Id, InputValueId as In, OutputValueId as Out, Value,
};

use crate::CalculationOutcome;

// TODO:
// Handle these calculations as usecases in the domain or application layer.
#[must_use]
pub fn calculate(
    input: &HashMap<Id, domain::Value>,
    custom_edges: Option<&[(Id, Id)]>,
    custom_leafs: Vec<Id>,
) -> CalculationOutcome {
    log::debug!("Calculate");

    let mut output = domain::calculate(input, custom_edges)
        .map_err(|err| log::warn!("{err}"))
        .ok();

    let custom_sum: Option<Tons> = output.clone().map(|(values, _)| {
        values
            .iter()
            .filter_map(|(id, value)| {
                if id.is_custom() && custom_leafs.iter().find(|&x| x == id).is_some() {
                    let v = value.clone().as_tons().unwrap_or_else(|| Tons::new(0.0));
                    Some(v)
                } else {
                    None
                }
            })
            .fold(Tons::new(0.0), |acc, element| acc + element)
    });

    if let Some((values, _)) = &mut output {
        if let Some(sum) = custom_sum {
            values.insert(Id::Out(Out::AdditionalCustomEmissions), Value::from(sum));
        }
    }

    log::debug!("Calculate all N2O emission factor scenarios");
    let maybe_graph = output.clone().map(|(_, graph)| graph).clone();

    let sensitivity_n2o_calculations =
        domain::calculate_all_n2o_emission_factor_scenarios(&input, maybe_graph.as_deref())
            .ok()
            .map(|results| {
                results
                    .into_iter()
                    .map(|(method, (values, _))| (method, values))
                    .collect()
            });

    let sensitivity_ch4_chp_calculations = {
        log::debug!("Calculate all CH4 CHP emission factor scenarios");

        let sewage_gas_produced = required!(In::SewageGasProduced, &input).unwrap();
        let methane_fraction = required!(In::MethaneFraction, &input).unwrap();
        let custom_ch4_chp_emission_factor = optional!(In::SensitivityCH4ChpCustomFactor, &input);
        let results = domain::calculate_all_ch4_chp_emission_factor_scenarios(
            sewage_gas_produced,
            methane_fraction,
            custom_ch4_chp_emission_factor,
        );
        Some(results)
    };

    CalculationOutcome {
        input: input.clone(),
        output: output.as_ref().map(|(values, _)| values.clone()),
        graph: output.as_ref().map(|(_, graph)| graph.clone()),
        sensitivity_n2o_calculations,
        sensitivity_ch4_chp_calculations,
    }
}
