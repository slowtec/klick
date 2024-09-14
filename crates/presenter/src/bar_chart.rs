use std::collections::HashMap;

use klick_domain::{
    units::{Percent, RatioExt, Tons},
    OutputValueId as Out, Value, ValueId as Id,
};

use crate::{Lng, ValueLabel};

#[must_use]
pub fn recommendation_diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
    lng: Lng,
) -> Vec<(String, f64, Option<f64>)> {
    let data_labels = [
        Out::Ch4SludgeBags,
        Out::Ch4SludgeStorageContainers,
        Out::Ch4Plant,
        Out::N2oSideStream,
        Out::FossilEnergySavings,
        Out::ProcessEnergySavings,
        Out::PhotovoltaicExpansionSavings,
        Out::DistrictHeatingSavings,
        Out::AdditionalCustomEmissions,
        Out::TotalEmissions,
    ];
    diff_bar_chart(old, new, &data_labels, lng)
}

#[must_use]
pub fn sensitivity_diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
    lng: Lng,
) -> Vec<(String, f64, Option<f64>)> {
    let data_labels = [
        Out::N2oPlant,
        Out::Ch4SludgeBags,
        Out::Ch4SludgeStorageContainers,
        Out::Ch4Plant,
        Out::Ch4CombinedHeatAndPowerPlant,
        Out::FossilEmissions,
        Out::N2oSideStream,
        Out::AdditionalCustomEmissions,
        Out::ElectricityMix,
        Out::TotalEmissions,
    ];
    diff_bar_chart(old, new, &data_labels, lng)
}

#[must_use]
fn diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
    data_labels: &[Out],
    lng: Lng,
) -> Vec<(String, f64, Option<f64>)> {
    let diff = calculate_difference(&new, &old);
    let total_emissions = new
        .get(&Out::TotalEmissions.into())
        .cloned()
        .and_then(Value::as_tons)
        .unwrap_or_else(Tons::zero);

    data_labels
        .iter()
        .map(|id| {
            let value = diff.get(&(*id).into()).copied().unwrap_or_else(Tons::zero);
            let percentage = if total_emissions != Tons::zero() {
                Some((value / total_emissions).convert_to::<Percent>())
            } else {
                None
            };
            (id.label(lng), value.into(), percentage.map(Into::into))
        })
        .collect()
}

fn calculate_difference(new: &HashMap<Id, Value>, old: &HashMap<Id, Value>) -> HashMap<Id, Tons> {
    let mut diff = HashMap::new();

    for key in new.keys() {
        let Some(new_val) = new.get(key).cloned().and_then(Value::as_tons) else {
            continue;
        };
        let old_val = old
            .get(key)
            .cloned()
            .and_then(Value::as_tons)
            .unwrap_or_else(Tons::zero);
        diff.insert(key.clone(), new_val - old_val);
    }
    diff
}
