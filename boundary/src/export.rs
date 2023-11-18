use serde::Serialize;

use crate::{InputData, Scenario, CURRENT_VERSION};

#[derive(Serialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone, PartialEq))]
struct Export<'a> {
    pub version: u32,
    pub input: &'a InputData,
    pub scenario: &'a Scenario,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_string_pretty(input: &InputData, scenario: &Scenario) -> String {
    let export = Export {
        version: CURRENT_VERSION,
        input,
        scenario,
    };
    serde_json::to_string_pretty(&export).expect("Valid input data")
}
