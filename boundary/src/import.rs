use serde::Deserialize;
use thiserror::Error;

use crate::{InputData, CURRENT_VERSION};

#[derive(Deserialize)]
struct VersionInfo {
    version: u32,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
struct Import {
    input: InputData,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected version {actual} (expected {expected})")]
    Version { actual: u32, expected: u32 },
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub fn import_from_str(json: &str) -> Result<InputData, Error> {
    let VersionInfo { version } = serde_json::from_str(json)?;

    if version != CURRENT_VERSION {
        return Err(Error::Version {
            actual: version,
            expected: CURRENT_VERSION,
        });
    }
    let import: Import = serde_json::from_str(json)?;
    Ok(import.input)
}
