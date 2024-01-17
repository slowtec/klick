use http::StatusCode;
use serde::{Deserialize, Serialize};

#[cfg(feature = "axum")]
mod axum;

pub mod login;
pub mod register;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error<T> {
    /// Short error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// HTTP status code
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,

    /// Structured error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<T>,
}

impl<T> Error<T> {
    pub const fn internal() -> Self {
        Self {
            message: None, // We really want to hide internal details
            status: StatusCode::INTERNAL_SERVER_ERROR,
            details: None, // We realld want to hide internal details
        }
    }
    pub const fn unauthorized() -> Self {
        Self {
            message: None,
            status: StatusCode::UNAUTHORIZED,
            details: None,
        }
    }
    pub const fn bad_request() -> Self {
        Self {
            message: None,
            status: StatusCode::BAD_REQUEST,
            details: None,
        }
    }
    pub fn message<S>(mut self, msg: S) -> Self
    where
        S: ToString,
    {
        self.message = Some(msg.to_string());
        self
    }
    pub fn details(mut self, details: T) -> Self {
        self.details = Some(details);
        self
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestPasswordReset {
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfirmEmailAddress {
    pub token: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResetPassword {
    pub token: String,
    pub new_password: String,
}
