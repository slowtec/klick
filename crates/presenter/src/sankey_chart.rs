use klick_domain::CO2Equivalents;

pub fn create_sankey_chart_data(
    co2_equivalents: CO2Equivalents,
) -> (Vec<(f64, &'static str, &'static str)>, Vec<(usize, usize)>) {
    let CO2Equivalents {
        n2o_plant,
        n2o_water,
        n2o_emissions,
        ch4_sewage_treatment,
        ch4_sludge_storage_containers,
        ch4_sludge_bags,
        ch4_water,
        ch4_combined_heat_and_power_plant,
        ch4_emissions,
        fecl3,
        feclso4,
        caoh2,
        synthetic_polymers,
        electricity_mix,
        operating_materials,
        sewage_sludge_transport,
        total_emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent: _,
    } = co2_equivalents;

    let mut nodes: Vec<(f64, &str, &str)> = vec![];

    let orange = "orange";
    nodes.push((indirect_emissions.into(), "Indirekte Emissionen", orange));
    let indirect_emissions = nodes.len() - 1;

    nodes.push((electricity_mix.into(), "Strommix", orange));
    let electricity_mix = nodes.len() - 1;

    let yellow = "#fd0";
    nodes.push((
        other_indirect_emissions.into(),
        "Weitere Indirekte Emissionen",
        yellow,
    ));
    let other_indirect_emissions = nodes.len() - 1;

    nodes.push((operating_materials.into(), "Betriebsstoffe", yellow));
    let operating_materials = nodes.len() - 1;

    nodes.push((fecl3.into(), "Eisen(III)-chlorid-Lösung", yellow));
    let fecl3 = nodes.len() - 1;

    nodes.push((feclso4.into(), "Eisenchloridsulfat-Lösung", yellow));
    let feclso4 = nodes.len() - 1;

    nodes.push((caoh2.into(), "Kalkhydrat", yellow));
    let caoh2 = nodes.len() - 1;

    nodes.push((synthetic_polymers.into(), "Synthetische Polymere", yellow));
    let synthetic_polymers = nodes.len() - 1;

    nodes.push((
        sewage_sludge_transport.into(),
        "Klaerschlamm Transport",
        yellow,
    ));
    let sewage_sludge_transport = nodes.len() - 1;

    let red = "red";
    nodes.push((total_emissions.into(), "Emission", red));
    let emissions = nodes.len() - 1;

    nodes.push((direct_emissions.into(), "Direkte Emissionen", red));
    let direct_emissions = nodes.len() - 1;

    nodes.push((n2o_emissions.into(), "Lachgasemissionen", red));
    let n2o_emissions = nodes.len() - 1;

    nodes.push((ch4_emissions.into(), "Methanemissionen", red));
    let ch4_emissions = nodes.len() - 1;

    nodes.push((n2o_plant.into(), "N₂O Anlage", red));
    let n2o_plant = nodes.len() - 1;

    nodes.push((n2o_water.into(), "N₂O Gewässer", red));
    let n2o_water = nodes.len() - 1;

    nodes.push((ch4_sewage_treatment.into(), "CH₄ Anlage (unspez.)", red));
    let ch4_sewage_treatment = nodes.len() - 1;

    nodes.push((
        ch4_sludge_storage_containers.into(),
        "CH₄ Schlupf Schlammlagerung",
        red,
    ));
    let ch4_sludge_storage_containers = nodes.len() - 1;

    nodes.push((ch4_sludge_bags.into(), "CH₄ Schlupf Schlammtasche", red));
    let ch4_sludge_bags = nodes.len() - 1;

    nodes.push((ch4_water.into(), "CH₄ Gewässer", red));
    let ch4_water = nodes.len() - 1;

    nodes.push((ch4_combined_heat_and_power_plant.into(), "CH₄ BHKW", red));
    let ch4_combined_heat_and_power_plant = nodes.len() - 1;

    let unfiltered_edges = [
        (fecl3, operating_materials),
        (synthetic_polymers, operating_materials),
        (sewage_sludge_transport, other_indirect_emissions),
        (feclso4, operating_materials),
        (caoh2, operating_materials),
        (n2o_plant, n2o_emissions),
        (n2o_water, n2o_emissions),
        (n2o_emissions, direct_emissions),
        (ch4_sewage_treatment, ch4_emissions),
        (ch4_sludge_storage_containers, ch4_emissions),
        (ch4_sludge_bags, ch4_emissions),
        (ch4_water, ch4_emissions),
        (ch4_combined_heat_and_power_plant, ch4_emissions),
        (ch4_emissions, direct_emissions),
        (electricity_mix, indirect_emissions),
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
