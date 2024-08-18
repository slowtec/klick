use std::collections::HashMap;

use klick_value::{specs::OutputValueId as Out, units::Tons};

pub fn calculate_emission_groups(
    mut values: HashMap<Out, Tons>,
    edges: &[(Out, Out)],
) -> HashMap<Out, Tons> {
    for &(source, target) in edges {
        let Some(value) = values.get(&source).copied() else {
            continue;
        };
        let target_value = values.entry(target).or_insert(Tons::zero());
        *target_value += value;
    }
    values
}
