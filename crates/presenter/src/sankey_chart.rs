use std::collections::HashMap;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    units::{Percent, RatioExt, Tons},
    InputValueId as Id, OutputValueId as Out, Value,
};

use crate::{Formatting, Lng};

#[must_use]
pub fn create_sankey_chart_header(
    data: &FormData,
    emission_factors: domain::CalculatedEmissionFactors,
    calculation_methods: domain::EmissionFactorCalculationMethods,
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
        f64::from(emission_factors.n2o.convert_to::<Percent>()),
        3,
    );

    let method = formatting.fmt_label(calculation_methods.n2o);

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
        (Out::N2oPlant, "N₂O Anlage", "red", "#ffb2b2"),
        (Out::N2oWater, "N₂O Gewässer", "red", "#ffb2b2"),
        (
            Out::N2oSideStream,
            "N₂O Prozesswasserbehandlung",
            "red",
            "#ffb2b2",
        ),
        (Out::N2oEmissions, "Lachgasemissionen", "red", "#ffb2b2"),
        (Out::Ch4Plant, "CH₄ Anlage", "red", "#ffb2b2"),
        (
            Out::Ch4SludgeStorageContainers,
            "CH₄ Schlamm Lagerung",
            "red",
            "#ffb2b2",
        ),
        (Out::Ch4SludgeBags, "CH₄ Schlammtasche", "red", "#ffb2b2"),
        (Out::Ch4Water, "CH₄ Gewässer", "red", "#ffb2b2"),
        (
            Out::Ch4CombinedHeatAndPowerPlant,
            "CH₄ BHKW",
            "red",
            "#ffb2b2",
        ),
        (Out::Ch4Emissions, "Methanemissionen", "red", "#ffb2b2"),
        (
            Out::FossilEmissions,
            "Fossile CO₂-Emissionen",
            "red",
            "#ffb2b2",
        ),
        (Out::Fecl3, "Eisen(III)-chlorid-Lösung", "yellow", "#fff5b2"),
        (
            Out::Feclso4,
            "Eisenchloridsulfat-Lösung",
            "yellow",
            "#fff5b2",
        ),
        (Out::Caoh2, "Kalkhydrat", "yellow", "#fff5b2"),
        (
            Out::SyntheticPolymers,
            "Synthetische Polymere",
            "yellow",
            "#fff5b2",
        ),
        (Out::ElectricityMix, "Strommix", "orange", "#ffe4b2"),
        (Out::OilEmissions, "Heizöl", "orange", "#ffe4b2"),
        (Out::GasEmissions, "Gas", "orange", "#ffe4b2"),
        (
            Out::OperatingMaterials,
            "Betriebsstoffe",
            "yellow",
            "#fff5b2",
        ),
        (
            Out::SewageSludgeTransport,
            "Klärschlamm Transport",
            "yellow",
            "#fff5b2",
        ),
        (Out::TotalEmissions, "Gesamtemissionen", "black", "#000000"),
        (Out::DirectEmissions, "Direkte Emissionen", "red", "#ffb2b2"),
        (
            Out::IndirectEmissions,
            "Indirekte Emissionen",
            "orange",
            "#ffe4b2",
        ),
        (
            Out::OtherIndirectEmissions,
            "Weitere Indirekte Emissionen",
            "yellow",
            "#fff5b2",
        ),
    ];

    let nodes = node_labels
        .iter()
        .map(|(id, label, color, color_lite)| {
            (
                f64::from(co2_equivalents.get(id).copied().unwrap_or_else(Tons::zero)),
                *label,
                *color,
                *color_lite,
            )
        })
        .collect();

    let edges = [
        (Out::Ch4SludgeBags, Out::Ch4Emissions),
        (Out::Ch4SludgeStorageContainers, Out::Ch4Emissions),
        (Out::Ch4Plant, Out::Ch4Emissions),
        (Out::Ch4Water, Out::Ch4Emissions),
        (Out::Ch4CombinedHeatAndPowerPlant, Out::Ch4Emissions),
        (Out::N2oPlant, Out::N2oEmissions),
        (Out::N2oWater, Out::N2oEmissions),
        (Out::N2oSideStream, Out::N2oEmissions),
        (Out::N2oEmissions, Out::DirectEmissions),
        (Out::Ch4Emissions, Out::DirectEmissions),
        (Out::FossilEmissions, Out::DirectEmissions),
        (Out::ElectricityMix, Out::IndirectEmissions),
        (Out::OilEmissions, Out::IndirectEmissions),
        (Out::GasEmissions, Out::IndirectEmissions),
        (Out::OperatingMaterials, Out::OtherIndirectEmissions),
        (Out::OtherIndirectEmissions, Out::TotalEmissions),
        (Out::DirectEmissions, Out::TotalEmissions),
        (Out::IndirectEmissions, Out::TotalEmissions),
    ];

    let mut connections: Vec<(usize, usize)> = Vec::new();
    for (from, to) in edges.iter() {
        let from_idx = node_labels
            .iter()
            .position(|(id, _, _, _)| id == from)
            .unwrap();
        let to_idx = node_labels
            .iter()
            .position(|(id, _, _, _)| id == to)
            .unwrap();
        connections.push((from_idx, to_idx));
    }

    (nodes, connections)
}
