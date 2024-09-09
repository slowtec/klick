use serde::Serialize;

use crate::{v9::Project, CURRENT_VERSION};

#[derive(Serialize)]
struct Export<'a> {
    pub version: u32,
    #[serde(flatten)]
    pub data: &'a Project,
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_string_pretty(data: &Project) -> String {
    let export = pack(data);
    serde_json::to_string_pretty(&export).expect("Valid input data")
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_string(data: &Project) -> String {
    let export = pack(data);
    serde_json::to_string(&export).expect("Valid input data")
}

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn export_to_vec_pretty(data: &Project) -> Vec<u8> {
    let export = pack(data);
    serde_json::to_vec_pretty(&export).expect("Valid input data")
}

fn pack(data: &Project) -> Export {
    Export {
        version: CURRENT_VERSION,
        data,
    }
}
