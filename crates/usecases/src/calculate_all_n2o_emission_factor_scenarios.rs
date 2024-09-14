use std::collections::HashMap;

use klick_domain::{units::*, InputValueId as In, Value as V, ValueId as Id};

use crate::{calculate, Edge, Edges, Values};

pub fn calculate_all_n2o_emission_factor_scenarios(
    values: &HashMap<Id, Value>,
    custom_edges: Option<&[Edge]>,
) -> anyhow::Result<Vec<(N2oEmissionFactorCalcMethod, (Values, Edges))>> {
    let mut values = values.clone();
    let id = In::SensitivityN2OCalculationMethod;

    // TuWien2016
    let n2o = N2oEmissionFactorCalcMethod::TuWien2016;
    values.insert(id.into(), V::n2o_emission_factor_calc_method(n2o));
    let result = calculate(&values, custom_edges)?;
    let tuwien2016_result = (n2o, result);

    // Optimistic
    let n2o = N2oEmissionFactorCalcMethod::Optimistic;
    values.insert(id.into(), V::n2o_emission_factor_calc_method(n2o));
    let result = calculate(&values, custom_edges)?;
    let optimistc_result = (n2o, result);

    // Pesimistic
    let n2o = N2oEmissionFactorCalcMethod::Pesimistic;
    values.insert(id.into(), V::n2o_emission_factor_calc_method(n2o));
    let result = calculate(&values, custom_edges)?;
    let pesimistic_result = (n2o, result);

    // Ipcc2019
    let n2o = N2oEmissionFactorCalcMethod::Ipcc2019;
    values.insert(id.into(), V::n2o_emission_factor_calc_method(n2o));
    let result = calculate(&values, custom_edges)?;
    let ipcc2019_result = (n2o, result);

    let mut results = vec![
        tuwien2016_result,
        optimistc_result,
        pesimistic_result,
        ipcc2019_result,
    ];

    // #306 sync custom defined scenarios between N2O and CH4, "on" by default
    // if !values.contains_key(&In::SensitivityN2OCustomFactor.into()) {
    //     return Ok(results);
    // };

    // Custom
    let n2o = N2oEmissionFactorCalcMethod::Custom;
    values.insert(id.into(), V::n2o_emission_factor_calc_method(n2o));
    let result = calculate(&values, custom_edges)?;
    let custom_result = (n2o, result);
    results.push(custom_result);

    Ok(results)
}
