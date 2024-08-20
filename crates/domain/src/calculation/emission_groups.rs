use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter,
    ops::AddAssign,
};

use klick_value::{
    specs::OutputValueId as Out,
    units::{Tons, Value},
};

pub const SANKEY_EDGES: &[(Out, Out)] = &[
    (Out::Ch4SludgeBags, Out::Ch4Emissions),
    (Out::Ch4SludgeStorageContainers, Out::Ch4Emissions),
    (Out::Ch4Plant, Out::Ch4Emissions),
    (Out::Ch4Water, Out::Ch4Emissions),
    (Out::Ch4CombinedHeatAndPowerPlant, Out::Ch4Emissions),
    (Out::N2oPlant, Out::N2oEmissions),
    (Out::N2oWater, Out::N2oEmissions),
    (Out::N2oSideStream, Out::N2oEmissions),
    (Out::SyntheticPolymers, Out::OperatingMaterials),
    (Out::Feclso4, Out::OperatingMaterials),
    (Out::Caoh2, Out::OperatingMaterials),
    (Out::Fecl3, Out::OperatingMaterials),
    (Out::N2oEmissions, Out::DirectEmissions),
    (Out::Ch4Emissions, Out::DirectEmissions),
    (Out::FossilEmissions, Out::DirectEmissions),
    (Out::ElectricityMix, Out::IndirectEmissions),
    (Out::OilEmissions, Out::IndirectEmissions),
    (Out::GasEmissions, Out::IndirectEmissions),
    (Out::OperatingMaterials, Out::OtherIndirectEmissions),
    (Out::SewageSludgeTransport, Out::OtherIndirectEmissions),
    (Out::OtherIndirectEmissions, Out::TotalEmissions),
    (Out::DirectEmissions, Out::TotalEmissions),
    (Out::IndirectEmissions, Out::TotalEmissions),
];

pub fn extract_emission_groups<ID>(
    values: &HashMap<ID, Value>,
    edges: &[(ID, ID)],
) -> HashMap<ID, Tons>
where
    ID: Eq + Hash + Clone,
{
    emission_group_ids(edges)
        .into_iter()
        .filter_map(|id| values.get(&id).cloned().map(|v| (id, v)))
        .filter_map(|(id, v)| v.as_tons().map(|v| (id, v)))
        .collect()
}

pub fn emission_group_ids<ID>(edges: &[(ID, ID)]) -> HashSet<ID>
where
    ID: Eq + Hash + Clone,
{
    edges
        .iter()
        .flat_map(|(source, target)| iter::once(source.clone()).chain(iter::once(target.clone())))
        .collect()
}

pub fn calculate_emission_groups<ID, T>(
    mut values: HashMap<ID, T>,
    edges: &[(ID, ID)],
) -> HashMap<ID, T>
where
    ID: Eq + Hash + Clone,
    T: Default + AddAssign + Clone,
{
    for (source, target) in edges {
        let Some(source_value) = values.get(&source).cloned() else {
            continue;
        };
        let target_value = values.entry(target.clone()).or_insert(Default::default());
        *target_value += source_value;
    }
    values
}
