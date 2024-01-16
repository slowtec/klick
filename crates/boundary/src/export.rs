use serde::Serialize;

use crate::{Data, CURRENT_VERSION};

#[derive(Serialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Clone))]
struct Export<'a> {
    pub version: u32,
    #[serde(flatten)]
    pub data: &'a Data,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_string_pretty(data: &Data) -> String {
    let export = Export {
        version: CURRENT_VERSION,
        data,
    };
    serde_json::to_string_pretty(&export).expect("Valid input data")
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_vec_pretty(data: &Data) -> Vec<u8> {
    let export = Export {
        version: CURRENT_VERSION,
        data,
    };
    serde_json::to_vec_pretty(&export).expect("Valid input data")
}
