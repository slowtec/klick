use std::collections::HashMap;

use klick_boundary::{import_from_slice, InputData, Scenario};

use crate::forms::FieldSignal;

use super::fields::{load_fields, FieldId};

pub fn load_example_field_signal_values(
    signals: &HashMap<FieldId, FieldSignal>,
) -> anyhow::Result<()> {
    let (input, scenario) = example_input_data();
    load_fields(signals, input, scenario)
}

const EXAMPLE_DATA: &[u8] = include_bytes!("example_data.json");

fn example_input_data() -> (InputData, Scenario) {
    // TODO: let csb_zu = 1045.0;
    // TODO: let p_zu = 9.9;
    // TODO: let p_ab = 0.4;
    // TODO: let gas_zusatz = 1_300_000.0;
    // TODO: let biogas = false;
    import_from_slice(EXAMPLE_DATA).unwrap()
}
