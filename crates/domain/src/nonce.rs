use std::{fmt, str::FromStr, string::FromUtf8Error};

use thiserror::Error;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{EmailAddress, EmailAddressParseError};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Nonce(Uuid);

impl Nonce {
    pub const STR_LEN: usize = 32;

    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Error)]
#[error("invalid nonce")]
pub struct NonceParseError;

impl FromStr for Nonce {
    type Err = NonceParseError;

    fn from_str(nonce_str: &str) -> Result<Self, Self::Err> {
        nonce_str
            .parse::<Uuid>()
            .map(Nonce)
            .map_err(|_| NonceParseError)
    }
}

impl fmt::Display for Nonce {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0.as_simple())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmailNonce {
    pub email: EmailAddress,
    pub nonce: Nonce,
}

pub type ActualTokenLen = usize;

#[derive(Debug, Error)]
pub enum EmailNonceDecodingError {
    #[error(transparent)]
    Bs58(#[from] bs58::decode::Error),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error("nonce is too short: {0} instead of {}", Nonce::STR_LEN)]
    TooShort(ActualTokenLen),
    #[error(transparent)]
    Parse(#[from] NonceParseError),
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressParseError),
}

impl EmailNonce {
    #[must_use]
    pub fn encode_to_string(&self) -> String {
        let nonce = self.nonce.to_string();
        debug_assert_eq!(Nonce::STR_LEN, nonce.len());
        let mut concat = String::with_capacity(self.email.as_str().len() + nonce.len());
        concat += self.email.as_str();
        concat += &nonce;
        bs58::encode(concat).into_string()
    }

    pub fn decode_from_str(encoded: &str) -> Result<Self, EmailNonceDecodingError> {
        let decoded = bs58::decode(encoded).into_vec()?;
        let mut concat = String::from_utf8(decoded)?;
        if concat.len() < Nonce::STR_LEN {
            return Err(EmailNonceDecodingError::TooShort(concat.len()));
        }
        let email_len = concat.len() - Nonce::STR_LEN;
        let nonce_slice: &str = &concat[email_len..];
        let nonce = nonce_slice.parse::<Nonce>()?;
        concat.truncate(email_len);
        let email = concat.parse()?;
        Ok(Self { email, nonce })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AccountToken {
    pub email_nonce: EmailNonce,
    pub expires_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_email_nonce() {
        let example = EmailNonce {
            email: "test@example.com".parse().unwrap(),
            nonce: Nonce::new(),
        };
        let encoded = example.encode_to_string();
        let decoded = EmailNonce::decode_from_str(&encoded).unwrap();
        assert_eq!(example, decoded);
    }

    #[test]
    fn decode_empty_email_nonce() {
        assert!(EmailNonce::decode_from_str("").is_err());
    }

    #[test]
    fn should_generate_unique_instances() {
        let n1 = Nonce::new();
        let n2 = Nonce::new();
        assert_ne!(n1, n2);
    }

    #[test]
    fn should_convert_from_to_string() {
        let n1 = Nonce::new();
        let s1 = n1.to_string();
        assert_eq!(Nonce::STR_LEN, s1.len());
        let n2 = s1.parse::<Nonce>().unwrap();
        assert_eq!(n1, n2);
        assert_eq!(s1, n2.to_string());
    }
}
