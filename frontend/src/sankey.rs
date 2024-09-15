use std::collections::HashMap;

use leptos::*;

use klick_domain::{Value, ValueId as Id};
use klick_presenter as presenter;

use klick_app_charts::{sankey_chart, Color, SankeyData};

#[allow(clippy::too_many_lines, clippy::needless_pass_by_value)]
#[component]
pub fn Sankey(
    data: HashMap<Id, Value>,
    graph: Vec<(Id, Id)>,
    lang: presenter::Lng,
) -> impl IntoView {
    let co2_equivalents = data;

    let (nodes, edges) = presenter::create_sankey_chart_data(co2_equivalents, &graph, lang);

    let mut sankey_data = SankeyData::new();
    let node_count = nodes.len();
    let node_ids: Vec<_> = nodes
        .into_iter()
        .map(|(value, label, color, edge_color)| {
            sankey_data.insert_node(
                value,
                label,
                Some(Color::new(color)),
                Some(Color::new(edge_color)),
            )
        })
        .collect();
    assert_eq!(node_ids.len(), node_count);

    for (from_idx, to_idx) in edges {
        let from = node_ids[from_idx];
        let to = node_ids[to_idx];
        sankey_data.insert_edge(from, to);
    }

    let aria_label = Some("Die ausgewerteten Eingabedaten werden in einem Sankey-Diagramm dargestellt. Das Diagramm zeigt die bilanzierten Treibhausgasemissionen als CO2-Äquivalente in Tonnen pro Jahr für die einzelnen Entstehungsorte / Quellen. Das Diagramm hat seitlich angeordnete 4 Ebenen die die Flusspfade des Diagramms verbinden und die sich wie folgt gliedern: bullet point ganz rechts sind die Gesamtemissionen als ein Balken zusammengefasst; bullet point mittig-recht sind drei Balken zu sehen, die direkte, indirekten und weitere indirekte Emissionen wiedergeben; bullet point mittig-link werden einige der direkten, indirekten und weiteren indirekten Emissionen, nochmals näher beschrieben als Lachgas- und Methan-basierte Emissionen als direkte Emission und Betriebsstoffe als weitere indirekte Emission. bullet point ganz links im Bild findet sich die Balken der einzelnen Emissionen wieder wie: N2O Anlage, N2O Gewässer, CH₄ Schlupf Schlammtasche, CH₄ Schlupf Schlammlagerung, CH₄ BHKW, CH₄ Gewässer, CO₂ Emissionen, Fossile CO₂-Emissionen, Strommix, Eisendreichloridlösung, Synthetische Polymere, Klärschlammtransport.".to_string());

    let number_format = |value| lang.format_number_with_fixed_precision(value, 0);
    let font_size = 16.0;

    let chart = sankey_chart(
        sankey_data,
        1200.0,
        800.0,
        number_format,
        font_size,
        aria_label,
    )
    .to_string();

    view! { <div inner_html = chart></div> }
}
