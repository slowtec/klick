use klick_svg_charts::{SankeyChart, SankeyData};
use leptos::*;

#[component]
fn MySankeyChart() -> impl IntoView {
    let mut sankey = SankeyData::new();

    let n2o_plant = 409.0497357374996;
    let n2o_water = 72.22835441250001;
    let n2o_emissions = 481.27809014999957;
    let ch4_sewage_treatment = 322.0;

    let ch4_sludge_storage_containers = 0.0;
    let ch4_sludge_bags = 15.694308000000003;
    let ch4_water = 25.38675594;
    let ch4_combined_heat_and_power_plant = 25.492320000000003;
    let ch4_emissions = 388.57338394;
    let fecl3 = 122.6475;
    let feclso4 = 0.0;
    let caoh2 = 0.0;
    let synthetic_polymers = 26.4;
    let electricity_mix = 359.1;
    let operating_materials = 149.04749999999999;
    let sewage_sludge_transport = 39.568938749999994;
    let emissions = 1417.5679128399995;
    let direct_emissions = 869.8514740899996;
    let indirect_emissions = 359.1;
    let other_indirect_emissions = 188.61643875;

    let orange = Some("orange");
    let indirect_emissions = sankey.node(indirect_emissions, "Indirekte Emissionen", orange);
    let electricity_mix = sankey.node(electricity_mix, "Strommix", orange);

    let yellow = Some("#fd0");
    let other_indirect_emissions = sankey.node(
        other_indirect_emissions,
        "Weitere Indirekte Emissionen",
        yellow,
    );
    let operating_materials = sankey.node(operating_materials, "Betriebsstoffe", yellow);
    let fecl3 = sankey.node(fecl3, "Eisen(III)-chlorid-Lösung", yellow);
    let feclso4 = sankey.node(feclso4, "Eisenchloridsulfat-Lösung", yellow);
    let caoh2 = sankey.node(caoh2, "Kalkhydrat", yellow);
    let synthetic_polymers = sankey.node(synthetic_polymers, "Synthetische Polymere", yellow);
    let sewage_sludge_transport =
        sankey.node(sewage_sludge_transport, "Klaerschlamm Transport", yellow);

    let red = Some("red");
    let emissions = sankey.node(emissions, "Emission", red);
    let direct_emissions = sankey.node(direct_emissions, "Direkte Emissionen", red);
    let n2o_emissions = sankey.node(n2o_emissions, "Lachgasemissionen", red);
    let ch4_emissions = sankey.node(ch4_emissions, "Methanemissionen", red);
    let n2o_plant = sankey.node(n2o_plant, "N₂O Anlage", red);
    let n2o_water = sankey.node(n2o_water, "N₂O Gewässer", red);

    let ch4_sewage_treatment = sankey.node(ch4_sewage_treatment, "CH₄ Klärprozess", red);
    let ch4_sludge_storage_containers = sankey.node(
        ch4_sludge_storage_containers,
        "CH₄ Schlupf Schlammstapel",
        red,
    );
    let ch4_sludge_bags = sankey.node(ch4_sludge_bags, "CH₄ Schlupf Schlammtasche", red);
    let ch4_water = sankey.node(ch4_water, "CH₄ Gewässer", red);
    let ch4_combined_heat_and_power_plant =
        sankey.node(ch4_combined_heat_and_power_plant, "CH₄ BHKW", red);

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
        sankey.edge(from, to);
    }

    let mut other_sankey = SankeyData::new();

    let c = other_sankey.node(1.0, "c", None);
    let a = other_sankey.node(5.0, "a", None);
    let b = other_sankey.node(2.0, "b", None);

    let e = other_sankey.node(5.0, "e", None);
    let d = other_sankey.node(5.0, "d", None);
    let g = other_sankey.node(7.0, "g", None);
    let h = other_sankey.node(8.0, "h", None);
    let i = other_sankey.node(10.0, "i", None);
    let j = other_sankey.node(5.0, "j", None);
    let k = other_sankey.node(23.0, "k", None);
    let f = other_sankey.node(5.0, "f", None);

    other_sankey.edge(a, g);
    other_sankey.edge(b, g);

    other_sankey.edge(c, h);
    other_sankey.edge(g, h);

    other_sankey.edge(d, i);
    other_sankey.edge(e, i);

    other_sankey.edge(f, j);

    other_sankey.edge(h, k);
    other_sankey.edge(i, k);
    other_sankey.edge(j, k);

    view! {
      <h1>"Sankey Chart Example"</h1>
      <SankeyChart
        sankey = { sankey }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.1}")
      />
      <SankeyChart
        sankey = { other_sankey }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.0}")
      />
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    log::info!("Start web application");
    mount_to_body(|| {
        view! { <MySankeyChart /> }
    });
}
