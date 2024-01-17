use serde::{Deserialize, Serialize};

pub use super::Credentials;

#[derive(Serialize, Deserialize)]
pub enum Error {
    Credentials,
    EmailNotConfirmed,
}
