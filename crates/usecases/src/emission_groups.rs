use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    iter,
    ops::AddAssign,
};

use klick_domain::{OutputValueId as Out, OutputValueId};

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
        let Some(source_value) = values.get(source).cloned() else {
            continue;
        };
        let target_value = values.entry(target.clone()).or_default();
        *target_value += source_value;
    }
    values
}

#[must_use]
pub fn get_all_internal_nodes() -> Vec<OutputValueId> {
    let set: HashSet<_> = SANKEY_EDGES.iter().map(|(_, target)| *target).collect();
    set.iter().copied().collect()
}
