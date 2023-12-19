use serde::Deserialize;
use thiserror::Error;

use crate::{v1, v2, v3, InputData, Scenario, CURRENT_VERSION};

mod migrate;

pub fn import_from_str(json: &str) -> Result<(InputData, Scenario)> {
    import_from_slice(json.as_bytes())
}

pub fn import_from_slice(slice: &[u8]) -> Result<(InputData, Scenario)> {
    let VersionInfo { version } = serde_json::from_slice(slice)?;
    let v3::Import { input, scenario } = match version {
        1 => {
            let data = import::<v1::Import>(slice)?;
            migrate::from_v1(data)
        }
        2 => {
            let data = import::<v2::Import>(slice)?;
            migrate::from_v2(data)
        }
        3 => import(slice)?,
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

fn import<V>(slice: &[u8]) -> Result<V>
where
    for<'de> V: Deserialize<'de>,
{
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
