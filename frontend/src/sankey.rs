use leptos::*;

use klick_application as app;
use klick_domain as domain;

use klick_svg_charts::{Color, SankeyChart, SankeyData};

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn format_large_number<T>(number: T) -> String
where
    T: Into<f64>,
{
    // Convert the f64 to u64
    let t = number.into().ceil();
    let u = t as u64;

    // Format the u64 as a string with a comma
    let formatted_string = format!("{u:0}");

    // Insert a comma at the appropriate position
    let comma_separated_string = formatted_string
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 3 == 0 {
                format!(".{c}")
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    comma_separated_string
}

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
#[component]
pub fn Sankey(data: app::Output) -> impl IntoView {
    log::debug!("Render sankey chart for {data:#?}");

    let app::Output {
        co2_equivalents,
        n2o_emission_factor: _,
    } = data;

    let domain::CO2Equivalents {
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
        emissions,
        direct_emissions,
        indirect_emissions,
        other_indirect_emissions,
        excess_energy_co2_equivalent: _,
    } = co2_equivalents;

    let mut sankey = SankeyData::new();

    let orange = Some(Color::new("orange"));
    let indirect_emissions =
        sankey.insert_node(indirect_emissions.into(), "Indirekte Emissionen", orange);
    let electricity_mix = sankey.insert_node(electricity_mix.into(), "Strommix", orange);

    let yellow = Some(Color::new("#fd0"));
    let other_indirect_emissions = sankey.insert_node(
        other_indirect_emissions.into(),
        "Weitere Indirekte Emissionen",
        yellow,
    );
    let operating_materials =
        sankey.insert_node(operating_materials.into(), "Betriebsstoffe", yellow);
    let fecl3 = sankey.insert_node(fecl3.into(), "Eisen(III)-chlorid-Lösung", yellow);
    let feclso4 = sankey.insert_node(feclso4.into(), "Eisenchloridsulfat-Lösung", yellow);
    let caoh2 = sankey.insert_node(caoh2.into(), "Kalkhydrat", yellow);
    let synthetic_polymers =
        sankey.insert_node(synthetic_polymers.into(), "Synthetische Polymere", yellow);
    let sewage_sludge_transport = sankey.insert_node(
        sewage_sludge_transport.into(),
        "Klaerschlamm Transport",
        yellow,
    );

    let red = Some(Color::new("red"));
    let emissions = sankey.insert_node(emissions.into(), "Emission", red);
    let direct_emissions = sankey.insert_node(direct_emissions.into(), "Direkte Emissionen", red);
    let n2o_emissions = sankey.insert_node(n2o_emissions.into(), "Lachgasemissionen", red);
    let ch4_emissions = sankey.insert_node(ch4_emissions.into(), "Methanemissionen", red);
    let n2o_plant = sankey.insert_node(n2o_plant.into(), "N₂O Anlage", red);
    let n2o_water = sankey.insert_node(n2o_water.into(), "N₂O Gewässer", red);

    let ch4_sewage_treatment =
        sankey.insert_node(ch4_sewage_treatment.into(), "CH₄ Klärprozess", red);
    let ch4_sludge_storage_containers = sankey.insert_node(
        ch4_sludge_storage_containers.into(),
        "CH₄ Schlupf Schlammstapel",
        red,
    );
    let ch4_sludge_bags =
        sankey.insert_node(ch4_sludge_bags.into(), "CH₄ Schlupf Schlammtasche", red);
    let ch4_water = sankey.insert_node(ch4_water.into(), "CH₄ Gewässer", red);
    let ch4_combined_heat_and_power_plant =
        sankey.insert_node(ch4_combined_heat_and_power_plant.into(), "CH₄ BHKW", red);

    let edges = [
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

    let filtered_edges: Vec<_> = edges
        .into_iter()
        .filter(|(from, to)| {
            sankey.node_value(from) > Some(0.0) && sankey.node_value(to) > Some(0.0)
        })
        .collect();

    for (from, to) in filtered_edges {
        sankey.insert_edge(from, to);
    }

    Some(view! {
      <SankeyChart
        sankey = { sankey }
        width = 1200.0
        height = 800.0
        number_format = |n| format_large_number(n)
        font_size = 16.0
      />
    })
}
