use std::{fmt, str::FromStr};

use mailparse::addrparse;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EmailAddress(String);

#[derive(Debug, Error)]
#[error("The given email address is invalid")]
pub struct ParseError;

impl FromStr for EmailAddress {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        addrparse(s)
            .ok()
            .and_then(|parsed| parsed.extract_single_info())
            .map(|single_info| Self(single_info.addr))
            .ok_or(ParseError)
    }
}

impl EmailAddress {
    pub const fn new_unchecked(address: String) -> Self {
        Self(address)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
