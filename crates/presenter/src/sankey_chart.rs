use std::collections::HashMap;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    units::{Factor, Percent, RatioExt, Tons},
    InputValueId as Id, OutputValueId as Out, Value,
};

use crate::{value_labels::ValueLabel, Formatting, Lng};

#[must_use]
pub fn create_sankey_chart_header(
    data: &FormData,
    emission_factors: HashMap<Out, Factor>,
    calculation_methods: HashMap<Out, Value>,
    formatting: Formatting,
) -> String {
    let population_equivalent = match &data
        .get(&Id::PopulationEquivalent)
        .cloned()
        .map(Value::as_count_unchecked)
        .map(u64::from)
    {
        Some(v) => format!("{v}"),
        None => String::new(),
    };

    let plant_name = match &data
        .get(&Id::PlantName)
        .cloned()
        .map(Value::as_text_unchecked)
    {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let emission_factor = Lng::De.format_number_with_fixed_precision(
        f64::from(
            emission_factors
                .get(&Out::N2oCalculatedEmissionFactor)
                .unwrap()
                .convert_to::<Percent>(),
        ),
        3,
    );

    let method = formatting.fmt_label(
        calculation_methods
            .get(&Out::N2oEmissionFactorCalcMethod)
            .cloned()
            .unwrap()
            .as_n2o_emission_factor_calc_method_unchecked(),
    );

    let n2o_label = match formatting {
        Formatting::Text => "N₂O",
        Formatting::LaTeX => r"N\textsubscript{2}O",
    };

    let co2_label = match formatting {
        Formatting::Text => "CO₂",
        Formatting::LaTeX => r"CO\textsubscript{2}",
    };

    format!(
        "{plant_name} ({population_equivalent} EW) / Treibhausgasemissionen [t {co2_label} Äquivalente/Jahr] - Szenario {method} ({n2o_label}-EF={emission_factor})"
    )
}

#[must_use]
pub fn create_sankey_chart_data(
    co2_equivalents: HashMap<Out, Tons>,
) -> (
    Vec<(f64, &'static str, &'static str, &'static str)>,
    Vec<(usize, usize)>,
) {
    let node_labels = [
        (Out::N2oPlant, "red", "#ffb2b2"),
        (Out::N2oWater, "red", "#ffb2b2"),
        (Out::N2oSideStream, "red", "#ffb2b2"),
        (Out::N2oEmissions, "red", "#ffb2b2"),
        (Out::Ch4Plant, "red", "#ffb2b2"),
        (Out::Ch4SludgeStorageContainers, "red", "#ffb2b2"),
        (Out::Ch4SludgeBags, "red", "#ffb2b2"),
        (Out::Ch4Water, "red", "#ffb2b2"),
        (Out::Ch4CombinedHeatAndPowerPlant, "red", "#ffb2b2"),
        (Out::Ch4Emissions, "red", "#ffb2b2"),
        (Out::FossilEmissions, "red", "#ffb2b2"),
        (Out::Fecl3, "yellow", "#fff5b2"),
        (Out::Feclso4, "yellow", "#fff5b2"),
        (Out::Caoh2, "yellow", "#fff5b2"),
        (Out::SyntheticPolymers, "yellow", "#fff5b2"),
        (Out::ElectricityMix, "orange", "#ffe4b2"),
        (Out::OilEmissions, "orange", "#ffe4b2"),
        (Out::GasEmissions, "orange", "#ffe4b2"),
        (Out::OperatingMaterials, "yellow", "#fff5b2"),
        (Out::SewageSludgeTransport, "yellow", "#fff5b2"),
        (Out::TotalEmissions, "black", "#000000"),
        (Out::DirectEmissions, "red", "#ffb2b2"),
        (Out::IndirectEmissions, "orange", "#ffe4b2"),
        (Out::OtherIndirectEmissions, "yellow", "#fff5b2"),
    ];

    let nodes = node_labels
        .iter()
        .map(|(id, color, color_lite)| {
            (
                f64::from(co2_equivalents.get(id).copied().unwrap_or_else(Tons::zero)),
                id.label(),
                *color,
                *color_lite,
            )
        })
        .collect();

    // FIXME:
    // The edges defined in `domain::SANKEY_EDGES`
    // should be the sames as defined here,
    // the follwing fails:
    //
    //    assert_eq!(edges, domain::SANKEY_EDGES);
    //
    // let edges = &[
    //    (Out::Ch4SludgeBags, Out::Ch4Emissions),
    //    (Out::Ch4SludgeStorageContainers, Out::Ch4Emissions),
    //    (Out::Ch4Plant, Out::Ch4Emissions),
    //    (Out::Ch4Water, Out::Ch4Emissions),
    //    (Out::Ch4CombinedHeatAndPowerPlant, Out::Ch4Emissions),
    //    (Out::N2oPlant, Out::N2oEmissions),
    //    (Out::N2oWater, Out::N2oEmissions),
    //    (Out::N2oSideStream, Out::N2oEmissions),
    //    (Out::N2oEmissions, Out::DirectEmissions),
    //    (Out::Ch4Emissions, Out::DirectEmissions),
    //    (Out::FossilEmissions, Out::DirectEmissions),
    //    (Out::ElectricityMix, Out::IndirectEmissions),
    //    (Out::OilEmissions, Out::IndirectEmissions),
    //    (Out::GasEmissions, Out::IndirectEmissions),
    //    (Out::OperatingMaterials, Out::OtherIndirectEmissions),
    //    (Out::OtherIndirectEmissions, Out::TotalEmissions),
    //    (Out::DirectEmissions, Out::TotalEmissions),
    //    (Out::IndirectEmissions, Out::TotalEmissions),
    //];

    let edges = domain::SANKEY_EDGES.iter().filter(|(source, target)| {
        let Some(source_value) = co2_equivalents.get(source) else {
            return false;
        };
        let Some(target_value) = co2_equivalents.get(target) else {
            return false;
        };
        *source_value != Tons::zero() && *target_value != Tons::zero()
    });

    let mut connections: Vec<(usize, usize)> = Vec::new();
    for (from, to) in edges {
        let from_idx = node_labels
            .iter()
            .position(|(id, _, _)| id == from)
            .unwrap();
        let to_idx = node_labels.iter().position(|(id, _, _)| id == to).unwrap();
        connections.push((from_idx, to_idx));
    }

    (nodes, connections)
}
