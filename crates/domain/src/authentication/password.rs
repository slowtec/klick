use std::{fmt, str::FromStr};

use thiserror::Error;

pub struct Password(String);

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "***")
    }
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Password(***)")
    }
}

#[derive(Debug, Clone)]
pub struct HashedPassword(String);

#[derive(Debug, Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ParseError {
    #[error("password is too short (min. {0}")]
    TooShort(usize),
    #[error("password is too long (max. {0}")]
    TooLong(usize),
    #[error("password does not contain a digit")]
    NoDigit,
    #[error("invalid password")]
    Invalid,
}

const MIN_LEN: usize = 3;
const MAX_LEN: usize = 255;

impl Password {
    pub const MIN_LEN: usize = MIN_LEN;
    pub const MAX_LEN: usize = MAX_LEN;

    #[must_use]
    #[allow(clippy::missing_panics_doc)] // Should never panic
    pub fn to_hashed(&self) -> HashedPassword {
        debug_assert!(validate(&self.0, Self::MIN_LEN, Self::MAX_LEN).is_ok());
        // Hashing with bcrypt only fails if the system number generator is
        // not available. A failure is never caused by the input.
        let hash =
            bcrypt::hash(&self.0, bcrypt::DEFAULT_COST).expect("bcrypt hash should never fail");
        HashedPassword::from_hash(hash)
    }
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(password: &str) -> Result<Self, Self::Err> {
        validate(password, Self::MIN_LEN, Self::MAX_LEN)?;
        Ok(Self(password.to_string()))
    }
}

impl HashedPassword {
    #[must_use]
    pub const fn from_hash(hashed: String) -> Self {
        Self(hashed)
    }

    #[must_use]
    pub fn verify(&self, password: &Password) -> bool {
        bcrypt::verify(&password.0, &self.0)
            .map_err(|err| {
                log::warn!("Unable to verify password: {err}");
            })
            .unwrap_or(false)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HashedPassword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

// TODO: Require digits and maybe more
const fn validate(password: &str, min_len: usize, max_len: usize) -> Result<(), ParseError> {
    if password.len() < min_len {
        return Err(ParseError::TooShort(min_len));
    }
    if password.len() > max_len {
        return Err(ParseError::TooLong(max_len));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_length() {
        let password = ["0"; MIN_LEN - 1].join("");
        let result = password.parse::<Password>();
        let err = result.err().unwrap();
        assert_eq!(err, ParseError::TooShort(MIN_LEN));

        let password = ["0"; MIN_LEN].join("");
        let result = password.parse::<Password>();
        assert!(result.is_ok());
    }

    #[test]
    fn max_length() {
        let password = ["x"; MAX_LEN + 1].join("");
        let result = password.parse::<Password>();
        let err = result.err().unwrap();
        assert_eq!(err, ParseError::TooLong(MAX_LEN));

        let password = ["0"; MAX_LEN].join("");
        let result = password.parse::<Password>();
        assert!(result.is_ok());
    }
}
