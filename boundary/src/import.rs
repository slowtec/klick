use serde::Deserialize;
use thiserror::Error;

use crate::{InputData, Scenario, CURRENT_VERSION};

#[derive(Deserialize)]
struct VersionInfo {
    version: u32,
}

#[derive(Deserialize)]
#[cfg_attr(feature = "extra-derive", derive(Debug, Default, Clone, PartialEq))]
struct Import {
    input: InputData,
    scenario: Scenario,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected version {actual} (expected {expected})")]
    Version { actual: u32, expected: u32 },
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub fn import_from_str(json: &str) -> Result<(InputData, Scenario), Error> {
    let version_info = serde_json::from_str(json)?;
    check_version(version_info)?;
    let Import { input, scenario } = serde_json::from_str(json)?;
    Ok((input, scenario))
}

pub fn import_from_slice(slice: &[u8]) -> Result<(InputData, Scenario), Error> {
    let version_info = serde_json::from_slice(slice)?;
    check_version(version_info)?;
    let Import { input, scenario } = serde_json::from_slice(slice)?;
    Ok((input, scenario))
}

#[allow(clippy::needless_pass_by_value)]
const fn check_version(info: VersionInfo) -> Result<(), Error> {
    let VersionInfo { version } = info;

    if version != CURRENT_VERSION {
        return Err(Error::Version {
            actual: version,
            expected: CURRENT_VERSION,
        });
    }
    Ok(())
}
