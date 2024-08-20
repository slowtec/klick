use std::collections::HashMap;

use klick_domain::{
    units::{Percent, RatioExt, Tons},
    Id, OutputValueId as Out, Value,
};

#[must_use]
pub fn recommendation_diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
) -> Vec<(&'static str, f64, Option<f64>)> {
    let data_labels = [
        (Out::Ch4SludgeBags, "CH₄ Schlupf Schlammtasche"),
        (
            Out::Ch4SludgeStorageContainers,
            "CH₄ Schlupf Schlammlagerung",
        ),
        (Out::Ch4Plant, "CH₄ Anlage (unspez.)"),
        (Out::N2oSideStream, "N₂O Prozesswasserbehandlung"),
        (Out::FossilEnergySavings, "Fossile Energiequellen"),
        (Out::ProcessEnergySavings, "Prozesse"),
        (Out::PhotovoltaicExpansionSavings, "Erneurbare Energien"),
        (Out::DistrictHeatingSavings, "Abwärme"),
        (Out::TotalEmissions, "Emissionen"),
    ];
    diff_bar_chart(old, new, &data_labels)
}

#[must_use]
pub fn sensitivity_diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
) -> Vec<(&'static str, f64, Option<f64>)> {
    let data_labels = [
        (Out::N2oPlant, "N₂O Anlage"),
        (Out::Ch4SludgeBags, "CH₄ Schlammtasche"),
        (Out::Ch4SludgeStorageContainers, "CH₄ Schlammlagerung"),
        (Out::Ch4Plant, "CH₄ Anlage (unspez.)"),
        (Out::Ch4CombinedHeatAndPowerPlant, "CH₄ BHKW"),
        (Out::FossilEmissions, "Fossiles CO₂"),
        (Out::N2oSideStream, "N₂O Prozesswasser"),
        (Out::TotalEmissions, "Emissionen"),
    ];
    diff_bar_chart(old, new, &data_labels)
}

#[must_use]
fn diff_bar_chart(
    old: HashMap<Id, Value>,
    new: HashMap<Id, Value>,
    data_labels: &[(Out, &'static str)],
) -> Vec<(&'static str, f64, Option<f64>)> {
    let diff = calculate_difference(&new, &old);
    let total_emissions = new
        .get(&Out::TotalEmissions.into())
        .cloned()
        .and_then(Value::as_tons)
        .unwrap_or_else(Tons::zero);

    data_labels
        .iter()
        .map(|(id, label)| {
            let value = diff.get(&(*id).into()).copied().unwrap_or_else(Tons::zero);
            let percentage = if total_emissions != Tons::zero() {
                Some((value / total_emissions).convert_to::<Percent>())
            } else {
                None
            };
            (*label, value.into(), percentage.map(Into::into))
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
