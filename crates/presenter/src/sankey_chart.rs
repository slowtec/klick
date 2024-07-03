use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    units::{Percent, RatioExt},
    CO2Equivalents, InputValueId as Id, Value,
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
        .map(Value::as_count_unchecked)
        .map(u64::from)
    {
        Some(v) => format!("{v}"),
        None => String::new(),
    };

    let plant_name = match &data.get(&Id::PlantName).map(Value::as_text_unchecked) {
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
    co2_equivalents: CO2Equivalents,
) -> (
    Vec<(f64, &'static str, &'static str, &'static str)>,
    Vec<(usize, usize)>,
) {
    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_side_stream,
        n2o_emissions,
        ch4_plant,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fossil_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        oil_emissions,
        gas_emissions,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        process_energy_savings: _,
        photovoltaic_expansion_savings: _,
        wind_expansion_savings: _,
        water_expansion_savings: _,
        district_heating_savings: _,
        fossil_energy_savings: _,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent: _,
    } = co2_equivalents;

    let mut nodes: Vec<(f64, &str, &str, &str)> = vec![];

    let orange = "orange";
    let orange_lite = "#ffe4b2";
    nodes.push((
        indirect_emissions.into(),
        "Indirekte Emissionen",
        orange,
        orange_lite,
    ));
    let indirect_emissions = nodes.len() - 1;

    nodes.push((electricity_mix.into(), "Strommix", orange, orange_lite));
    let electricity_mix = nodes.len() - 1;

    nodes.push((oil_emissions.into(), "Heizöl", orange, orange_lite));
    let oil_emissions = nodes.len() - 1;

    nodes.push((gas_emissions.into(), "Gas", orange, orange_lite));
    let gas_emissions = nodes.len() - 1;

    let yellow = "#fd0";
    let yellow_lite = "#fff5b2";

    nodes.push((
        other_indirect_emissions.into(),
        "Weitere Indirekte Emissionen",
        yellow,
        yellow_lite,
    ));
    let other_indirect_emissions = nodes.len() - 1;

    nodes.push((
        operating_materials.into(),
        "Betriebsstoffe",
        yellow,
        yellow_lite,
    ));
    let operating_materials = nodes.len() - 1;

    nodes.push((
        fecl3.into(),
        "Eisen(III)-chlorid-Lösung",
        yellow,
        yellow_lite,
    ));
    let fecl3 = nodes.len() - 1;

    nodes.push((
        feclso4.into(),
        "Eisenchloridsulfat-Lösung",
        yellow,
        yellow_lite,
    ));
    let feclso4 = nodes.len() - 1;

    nodes.push((caoh2.into(), "Kalkhydrat", yellow, yellow_lite));
    let caoh2 = nodes.len() - 1;

    nodes.push((
        synthetic_polymers.into(),
        "Synthetische Polymere",
        yellow,
        yellow_lite,
    ));
    let synthetic_polymers = nodes.len() - 1;

    nodes.push((
        sewage_sludge_transport.into(),
        "Klärschlamm Transport",
        yellow,
        yellow_lite,
    ));
    let sewage_sludge_transport = nodes.len() - 1;

    let red = "red";
    let red_lite = "#ffb2b2";
    nodes.push((total_emissions.into(), "Emission", red, red_lite));
    let emissions = nodes.len() - 1;

    nodes.push((direct_emissions.into(), "Direkte Emissionen", red, red_lite));
    let direct_emissions = nodes.len() - 1;

    nodes.push((n2o_emissions.into(), "Lachgasemissionen", red, red_lite));
    let n2o_emissions = nodes.len() - 1;

    nodes.push((ch4_emissions.into(), "Methanemissionen", red, red_lite));
    let ch4_emissions = nodes.len() - 1;

    nodes.push((n2o_plant.into(), "N₂O Anlage", red, red_lite));
    let n2o_plant = nodes.len() - 1;

    nodes.push((
        n2o_side_stream.into(),
        "N₂O Prozesswasserbehandlung",
        red,
        red_lite,
    ));
    let n2o_side_stream = nodes.len() - 1;

    nodes.push((n2o_water.into(), "N₂O Gewässer", red, red_lite));
    let n2o_water = nodes.len() - 1;

    nodes.push((ch4_plant.into(), "CH₄ Anlage (unspez.)", red, red_lite));
    let ch4_plant = nodes.len() - 1;

    nodes.push((
        ch4_sludge_storage_containers.into(),
        "CH₄ Schlupf Schlammlagerung",
        red,
        red_lite,
    ));
    let ch4_sludge_storage_containers = nodes.len() - 1;

    nodes.push((
        ch4_sludge_bags.into(),
        "CH₄ Schlupf Schlammtasche",
        red,
        red_lite,
    ));
    let ch4_sludge_bags = nodes.len() - 1;

    nodes.push((ch4_water.into(), "CH₄ Gewässer", red, red_lite));
    let ch4_water = nodes.len() - 1;

    nodes.push((
        ch4_combined_heat_and_power_plant.into(),
        "CH₄ BHKW",
        red,
        red_lite,
    ));
    let ch4_combined_heat_and_power_plant = nodes.len() - 1;

    nodes.push((
        fossil_emissions.into(),
        "Fossile CO₂-Emissionen",
        red,
        red_lite,
    ));
    let fossil_emissions = nodes.len() - 1;

    let unfiltered_edges = [
        (fecl3, operating_materials),
        (synthetic_polymers, operating_materials),
        (sewage_sludge_transport, other_indirect_emissions),
        (feclso4, operating_materials),
        (caoh2, operating_materials),
        (n2o_plant, n2o_emissions),
        (n2o_side_stream, n2o_emissions),
        (n2o_water, n2o_emissions),
        (n2o_emissions, direct_emissions),
        (ch4_plant, ch4_emissions),
        (ch4_sludge_storage_containers, ch4_emissions),
        (ch4_sludge_bags, ch4_emissions),
        (ch4_water, ch4_emissions),
        (ch4_combined_heat_and_power_plant, ch4_emissions),
        (ch4_emissions, direct_emissions),
        (fossil_emissions, direct_emissions),
        (electricity_mix, indirect_emissions),
        (oil_emissions, indirect_emissions),
        (gas_emissions, indirect_emissions),
        (operating_materials, other_indirect_emissions),
        (other_indirect_emissions, emissions),
        (direct_emissions, emissions),
        (indirect_emissions, emissions),
    ];

    let filtered_edges: Vec<_> = unfiltered_edges
        .into_iter()
        .filter(|(from, to)| nodes[*from].0 > 0.0 && nodes[*to].0 > 0.0)
        .collect();

    (nodes, filtered_edges)
}
