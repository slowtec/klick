use leptos::*;

use klick_app_charts::{Color, SankeyChart, SankeyData};

#[component]
fn MySankeyChart() -> impl IntoView {
    let mut sankey = SankeyData::new();
    let operating_materials = 149.04749999999999;

    let fecl3 = 122.6475;
    let feclso4 = 0.0;
    let caoh2 = 0.0;
    let foobar_alpha = 40.0;
    let foobar_beta = 60.0;
    let foobar_gamma = 22.6;
    let synthetic_polymers = 26.4;
    let electricity_mix = 159.1;
    let sewage_sludge_transport = 39.568938749999994;
    let emissions = 347.0;
    let indirect_emissions = 159.1;
    let other_indirect_emissions = 188.61643875;

    let indirect_emissions =
        sankey.insert_node(indirect_emissions, "Indirekte Emissionen", None, None);
    let electricity_mix = sankey.insert_node(electricity_mix, "Strommix", None, None);

    let other_indirect_emissions = sankey.insert_node(
        other_indirect_emissions,
        "Weitere Indirekte Emissionen",
        None,
        None,
    );
    let operating_materials = sankey.insert_node(operating_materials, "Betriebsstoffe", None, None);
    let fecl3 = sankey.insert_node(fecl3, "Eisen(III)-chlorid-Lösung", None, None);
    let feclso4 = sankey.insert_node(feclso4, "Eisenchloridsulfat-Lösung", None, None);
    let caoh2 = sankey.insert_node(caoh2, "Kalkhydrat", None, None);
    let foobar_alpha = sankey.insert_node(foobar_alpha, "Foobar alpha Polymere1", None, None);
    let foobar_beta = sankey.insert_node(foobar_beta, "Foobar beta Polymere", None, None);
    let foobar_gamma = sankey.insert_node(foobar_gamma, "Foobar gamma Polymere", None, None);

    let synthetic_polymers =
        sankey.insert_node(synthetic_polymers, "Synthetische Polymere", None, None);
    let sewage_sludge_transport =
        sankey.insert_node(sewage_sludge_transport, "Klärschlamm Transport", None, None);

    let emissions = sankey.insert_node(emissions, "Emission", None, None);

    let edges = [
        (foobar_gamma, fecl3),
        (foobar_alpha, fecl3),
        (foobar_beta, fecl3),
        (fecl3, operating_materials),
        (sewage_sludge_transport, other_indirect_emissions),
        (feclso4, operating_materials),
        (caoh2, operating_materials),
        (electricity_mix, indirect_emissions),
        (operating_materials, other_indirect_emissions),
        (other_indirect_emissions, emissions),
        (indirect_emissions, emissions),
        (synthetic_polymers, operating_materials),
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

    let mut other_sankey = SankeyData::new();

    let red = Some(Color::new("red"));
    let red_lite = Some(Color::new("#ffb2b2"));

    let c = other_sankey.insert_node(1.0, "c", red, red_lite);
    let a = other_sankey.insert_node(5.0, "a", red, red_lite);
    let b = other_sankey.insert_node(2.0, "b", red, red_lite);
    let g = other_sankey.insert_node(7.0, "g", red, red_lite);
    let h = other_sankey.insert_node(8.0, "h", red, red_lite);

    let orange = Some(Color::new("orange"));
    let orange_lite = Some(Color::new("#ffe4b2"));

    let e = other_sankey.insert_node(5.0, "e", orange, orange_lite);
    let d = other_sankey.insert_node(5.0, "d", orange, orange_lite);
    let i = other_sankey.insert_node(10.0, "i", orange, orange_lite);

    let yellow = Some(Color::new("#fd0"));
    let yellow_lite = Some(Color::new("#fff5b2"));

    let j = other_sankey.insert_node(5.0, "j", yellow, yellow_lite);
    let f = other_sankey.insert_node(5.0, "f", yellow, yellow_lite);
    let z = other_sankey.insert_node(1.0, "p", yellow, yellow_lite);

    let k = other_sankey.insert_node(24.0, "k", red, Some(Color::new("grey")));

    other_sankey.insert_edge(a, g);
    other_sankey.insert_edge(b, g);
    other_sankey.insert_edge(c, h);
    other_sankey.insert_edge(g, h);
    other_sankey.insert_edge(h, k);

    other_sankey.insert_edge(d, i);
    other_sankey.insert_edge(e, i);
    other_sankey.insert_edge(i, k);

    other_sankey.insert_edge(f, j);
    other_sankey.insert_edge(j, k);
    other_sankey.insert_edge(z, k);

    let mut short_sankey = SankeyData::new();
    let b = short_sankey.insert_node(20.0, "b", None, None);
    let a = short_sankey.insert_node(10.0, "a", None, None);
    let c = short_sankey.insert_node(30.0, "c", None, None);

    short_sankey.insert_edge(b, c);
    short_sankey.insert_edge(a, c);

    let mut short_sankey_w = SankeyData::new();
    let b = short_sankey_w.insert_node(10.0, "b", None, None);
    let a = short_sankey_w.insert_node(20.0, "a", None, None);
    let c = short_sankey_w.insert_node(30.0, "c", None, None);

    short_sankey_w.insert_edge(b, c);
    short_sankey_w.insert_edge(a, c);

    view! {
      <h1>"Sankey Chart Example"</h1>
      <SankeyChart
        sankey_data = { sankey }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.1}")
        font_size = 16.0
        aria_label = None
      />
      <SankeyChart
        sankey_data = { other_sankey }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.0}")
        font_size = 16.0
        aria_label = None
      />
      <SankeyChart
        sankey_data = { short_sankey }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.0}")
        font_size = 16.0
        aria_label = None
      />
      <SankeyChart
        sankey_data = { short_sankey_w }
        width = 800.0
        height = 300.0
        number_format = |n| format!("{n:.0}")
        font_size = 16.0
        aria_label = None
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
