use std::collections::HashMap;

use leptos::*;

use klick_domain::{Id, Value};
use klick_presenter as presenter;

use klick_app_charts::{Color, SankeyChart, SankeyData};

// FIXME: can we use Lng::format_number_with_thousands_seperator?
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
pub fn Sankey(data: HashMap<Id, Value>) -> impl IntoView {
    let co2_equivalents = data;

    let (nodes, edges) = presenter::create_sankey_chart_data(co2_equivalents);

    let mut sankey = SankeyData::new();
    let node_count = nodes.len();
    let node_ids: Vec<_> = nodes
        .into_iter()
        .map(|(value, label, color, edge_color)| {
            sankey.insert_node(
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
        sankey.insert_edge(from, to);
    }

    Some(view! {
        <SankeyChart
            sankey = { sankey }
            width = 1200.0
            height = 800.0
            number_format = format_large_number
            font_size = 16.0
            aria_label = Some("Die ausgewerteten Eingabedaten werden in einem Sankey-Diagramm dargestellt. Das Diagramm zeigt die bilanzierten Treibhausgasemissionen als CO2-Äquivalente in Tonnen pro Jahr für die einzelnen Entstehungsorte / Quellen. Das Diagramm hat seitlich angeordnete 4 Ebenen die die Flusspfade des Diagramms verbinden und die sich wie folgt gliedern: bullet point ganz rechts sind die Gesamtemissionen als ein Balken zusammengefasst; bullet point mittig-recht sind drei Balken zu sehen, die direkte, indirekten und weitere indirekte Emissionen wiedergeben; bullet point mittig-link werden einige der direkten, indirekten und weiteren indirekten Emissionen, nochmals näher beschrieben als Lachgas- und Methan-basierte Emissionen als direkte Emission und Betriebsstoffe als weitere indirekte Emission. bullet point ganz links im Bild findet sich die Balken der einzelnen Emissionen wieder wie: N2O Anlage, N2O Gewässer, CH₄ Schlupf Schlammtasche, CH₄ Schlupf Schlammlagerung, CH₄ BHKW, CH₄ Gewässer, CO₂ Emissionen, Fossile CO₂-Emissionen, Strommix, Eisendreichloridlösung, Synthetische Polymere, Klärschlammtransport.".to_string())
        />
    })
}
