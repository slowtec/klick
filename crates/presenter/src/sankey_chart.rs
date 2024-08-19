use std::collections::HashMap;

use derive_more::From;

use klick_boundary::FormData;
use klick_domain::{
    self as domain,
    output_value::*,
    units::{Percent, RatioExt, Tons},
    InputValueId as Id, OutputValueId as Out, Value,
};

use crate::{Formatting, Lng, ValueColor, ValueLabel};

#[must_use]
pub fn create_sankey_chart_header(
    data: &FormData,
    values: HashMap<Out, Value>,
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
            required!(Out::N2oCalculatedEmissionFactor, values)
                .unwrap()
                .convert_to::<Percent>(),
        ),
        3,
    );

    let method = formatting.fmt_label(required!(Out::N2oEmissionFactorCalcMethod, values).unwrap());

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

#[derive(Debug, Clone, PartialEq)]
pub struct SankeyTweaks {
    pub nodes: HashMap<String, (Tons, &'static str, &'static str)>,
    pub edges: Vec<(SankeyTweakId, SankeyTweakId)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum SankeyTweakId {
    Custom(String),
    Out(Out),
}

type Nodes = Vec<(f64, String, &'static str, &'static str)>;

#[must_use]
pub fn create_sankey_chart_data(
    co2_equivalents: HashMap<Out, Tons>,
    custom: Option<SankeyTweaks>,
) -> (Nodes, Vec<(usize, usize)>) {
    let node_ids = [
        Out::N2oPlant,
        Out::N2oWater,
        Out::N2oSideStream,
        Out::N2oEmissions,
        Out::Ch4Plant,
        Out::Ch4SludgeStorageContainers,
        Out::Ch4SludgeBags,
        Out::Ch4Water,
        Out::Ch4CombinedHeatAndPowerPlant,
        Out::Ch4Emissions,
        Out::FossilEmissions,
        Out::Fecl3,
        Out::Feclso4,
        Out::Caoh2,
        Out::SyntheticPolymers,
        Out::ElectricityMix,
        Out::OilEmissions,
        Out::GasEmissions,
        Out::OperatingMaterials,
        Out::SewageSludgeTransport,
        Out::TotalEmissions,
        Out::DirectEmissions,
        Out::IndirectEmissions,
        Out::OtherIndirectEmissions,
    ];

    let mut nodes = node_ids
        .iter()
        .map(|id| {
            (
                SankeyTweakId::from(*id),
                (
                    f64::from(co2_equivalents.get(id).copied().unwrap_or_else(Tons::zero)),
                    id.label().to_string(),
                    id.color(),
                    id.color_light(),
                ),
            )
        })
        .collect::<Vec<_>>();

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

    let mut edges = domain::SANKEY_EDGES
        .iter()
        .filter(|(source, target)| {
            let Some(source_value) = co2_equivalents.get(source) else {
                return false;
            };
            let Some(target_value) = co2_equivalents.get(target) else {
                return false;
            };
            *source_value != Tons::zero() && *target_value != Tons::zero()
        })
        .map(|(source, target)| (SankeyTweakId::from(*source), SankeyTweakId::from(*target)))
        .collect::<Vec<_>>();

    if let Some(tweaks) = custom {
        for (id, (value, color, color_lite)) in tweaks.nodes {
            nodes.push((
                SankeyTweakId::from(id.clone()),
                (f64::from(value), id, color, color_lite),
            ));
        }

        edges.extend(tweaks.edges.clone());

        let tweak_edges = tweaks
            .edges
            .into_iter()
            .map(|(s, t)| (s.into(), t.into()))
            .collect::<Vec<(SankeyTweakId, SankeyTweakId)>>();

        let values: HashMap<_, Tons> = nodes
            .iter()
            .map(|(id, (v, _, _, _))| (id.clone(), Tons::new(*v)))
            .collect();
        let tweaked_values = domain::calculate_emission_groups(values, &tweak_edges);

        for (node_id, value) in tweaked_values {
            let Some((_, (node, _, _, _))) = nodes.iter_mut().find(|(id, _)| *id == node_id) else {
                continue;
            };
            // NOTE:
            // We assume that all custom nodes already have their calculed
            // value, so we just tweak the `Out` values.
            if matches!(node_id, SankeyTweakId::Out(_)) {
                *node = f64::from(value);
            }
        }
    }

    let mut connections: Vec<(usize, usize)> = Vec::new();

    for (from, to) in edges {
        let from_idx = nodes.iter().position(|(id, _)| *id == from).unwrap();
        let to_idx = nodes.iter().position(|(id, _)| *id == to).unwrap();
        connections.push((from_idx, to_idx));
    }

    let nodes = nodes.into_iter().map(|(_, node)| node).collect();
    (nodes, connections)
}
