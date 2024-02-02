use serde::Deserialize;
use thiserror::Error;

use crate::{v1, v2, v3, v4, v5, v6, Project, CURRENT_VERSION};

mod migrate;

pub fn import_from_str(json: &str) -> Result<Project> {
    import_from_slice(json.as_bytes())
}

pub fn import_from_slice(slice: &[u8]) -> Result<Project> {
    let VersionInfo { version } = serde_json::from_slice(slice)?;
    let v6::Data { project } = match version {
        1 => {
            let v1 = import::<v1::Import>(slice)?;
            let v2 = migrate::from_v1(v1);
            let v3 = migrate::from_v2(v2);
            let v4 = migrate::from_v3(v3);
            let v5 = migrate::from_v4(v4);
            migrate::from_v5(v5)
        }
        2 => {
            let v2 = import::<v2::Import>(slice)?;
            let v3 = migrate::from_v2(v2);
            let v4 = migrate::from_v3(v3);
            let v5 = migrate::from_v4(v4);
            migrate::from_v5(v5)
        }
        3 => {
            let v3 = import::<v3::Import>(slice)?;
            let v4 = migrate::from_v3(v3);
            let v5 = migrate::from_v4(v4);
            migrate::from_v5(v5)
        }
        4 => {
            let v4 = import::<v4::Import>(slice)?;
            let v5 = migrate::from_v4(v4);
            migrate::from_v5(v5)
        }
        5 => {
            let v5 = import::<v5::Data>(slice)?;
            migrate::from_v5(v5)
        }
        6 => import(slice)?,
        _ => {
            return Err(Error::Version {
                actual: version,
                expected: CURRENT_VERSION,
            });
        }
    };
    Ok(project)
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
