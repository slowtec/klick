use std::collections::HashMap;

use derive_more::From;

use klick_domain::{
    self as domain,
    output_value::*,
    units::{Percent, RatioExt, Tons},
    Id, InputValueId as In, OutputValueId as Out, Value,
};

use crate::{Formatting, Lng, ValueColor, ValueLabel};

#[must_use]
pub fn create_sankey_chart_header(
    data: &HashMap<Id, Value>,
    values: HashMap<Id, Value>,
    formatting: Formatting,
    lang: Lng,
) -> String {
    let population_equivalent = match &data
        .get(&In::PopulationEquivalent.into())
        .cloned()
        .map(Value::as_count_unchecked)
        .map(u64::from)
    {
        Some(v) => format!("{v}"),
        None => String::new(),
    };

    let plant_name = match &data
        .get(&In::PlantName.into())
        .cloned()
        .map(Value::as_text_unchecked)
    {
        Some(v) => v.to_string(),
        None => String::new(),
    };

    let emission_factor = lang.format_number_with_fixed_precision(
        f64::from(
            required!(Out::N2oCalculatedEmissionFactor, values)
                .unwrap()
                .convert_to::<Percent>(),
        ),
        3,
    );

    let method = formatting.fmt_label(
        required!(Out::N2oEmissionFactorCalcMethod, values).unwrap(),
        lang,
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

type Nodes = Vec<(f64, String, &'static str, &'static str)>;

fn resolve_color(graph: &[(Id, Id)], pos: &Id) -> Option<(&'static str, &'static str)> {
    let is_in_graph = graph.iter().any(|(s, t)| s == pos || t == pos);
    if !is_in_graph {
        return None;
    }
    let parent: &Id = graph.iter().find(|(s, _)| s == pos).map(|(_, t)| t)?;
    match parent {
        Id::Out(out_id) => Some((out_id.color(), out_id.color_light())),
        x @ Id::Custom(_) => resolve_color(graph, x),
        _ => None,
    }
}

#[must_use]
pub fn create_sankey_chart_data(
    co2_equivalents: HashMap<Id, Value>,
    graph: &[(Id, Id)],
    lang: Lng,
) -> (Nodes, Vec<(usize, usize)>) {
    let node_ids = domain::emission_group_ids(graph);

    let nodes = node_ids
        .iter()
        .filter_map(|id| {
            let value = co2_equivalents
                .get(&id)
                .cloned()
                .and_then(Value::as_tons)
                .unwrap_or_else(Tons::zero);

            let (label, color, color_light) = match id {
                x @ Id::Custom(label) => resolve_color(graph, &x)
                    .map(|(color, color_light)| (label.clone(), color, color_light))
                    .unwrap_or_else(|| (label.clone(), "black", "grey")),
                Id::Out(id) => (id.label(lang).to_string(), id.color(), id.color_light()),
                Id::In(_) => {
                    return None;
                }
            };

            Some((id, (f64::from(value), label, color, color_light)))
        })
        .collect::<Vec<_>>();

    let filtered_edges = graph
        .iter()
        .filter(|(source, target)| {
            let Some(source_value) = co2_equivalents.get(&source) else {
                return false;
            };
            let Some(target_value) = co2_equivalents.get(&target) else {
                return false;
            };
            *source_value != Tons::zero().into() && *target_value != Tons::zero().into()
        })
        .collect::<Vec<_>>();

    let mut connections: Vec<(usize, usize)> = Vec::new();

    for (from, to) in filtered_edges {
        let from_idx = nodes.iter().position(|(id, _)| *id == from).unwrap();
        let to_idx = nodes.iter().position(|(id, _)| *id == to).unwrap();
        connections.push((from_idx, to_idx));
    }

    let nodes = nodes.into_iter().map(|(_, node)| node).collect();
    (nodes, connections)
}
