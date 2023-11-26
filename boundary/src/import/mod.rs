use serde::Deserialize;
use thiserror::Error;

use crate::{v2, InputData, Scenario, CURRENT_VERSION};

mod v1;

mod migrate;

pub fn import_from_str(json: &str) -> Result<(InputData, Scenario)> {
    import_from_slice(json.as_bytes())
}

pub fn import_from_slice(slice: &[u8]) -> Result<(InputData, Scenario)> {
    let VersionInfo { version } = serde_json::from_slice(slice)?;
    let v2::Import { input, scenario } = match version {
        1 => {
            let data = import_v1(slice)?;
            migrate::from_v1(data)
        }
        2 => import_v2(slice)?,
        _ => {
            return Err(Error::Version {
                actual: version,
                expected: CURRENT_VERSION,
            });
        }
    };
    Ok((input, scenario))
}

#[derive(Deserialize)]
struct VersionInfo {
    version: u32,
}

fn import_v1(slice: &[u8]) -> Result<v1::Import> {
    Ok(serde_json::from_slice(slice)?)
}

fn import_v2(slice: &[u8]) -> Result<v2::Import> {
    Ok(serde_json::from_slice(slice)?)
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected version {actual} (expected {expected})")]
    Version { actual: u32, expected: u32 },
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
