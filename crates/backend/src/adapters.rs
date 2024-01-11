use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;

use klick_application::usecases;
use klick_boundary::json_api;
use klick_domain::{
    EmailAddress, EmailAddressParseError, EmailNonceDecodingError, Password, PasswordParseError,
};

/// API error
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error(transparent)]
    CreateAccount(#[from] usecases::CreateAccountError),
    #[error(transparent)]
    CreateAccountEmail(EmailAddressParseError),
    #[error(transparent)]
    CreateAccountPassword(PasswordParseError),
    #[error(transparent)]
    Login(#[from] usecases::LoginError),
    #[error(transparent)]
    LoginEmail(EmailAddressParseError),
    #[error(transparent)]
    LoginPassword(PasswordParseError),
    #[error(transparent)]
    Logout(#[from] LogoutError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    EmailNonce(#[from] EmailNonceDecodingError),
    #[error(transparent)]
    ConfirmEmail(#[from] usecases::ConfirmEmailAddressError),
    #[error(transparent)]
    ResetPassword(#[from] usecases::ResetPasswordError),
}

pub struct Credentials {
    pub email: EmailAddress,
    pub password: Password,
}

#[derive(Debug, Error)]
pub enum LogoutError {
    #[error("you are not logged in")]
    NotLoggedIn,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("you are not authorized")]
    NotAuthorized,
}

#[derive(Debug, Error)]
pub enum CredentialParsingError {
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressParseError),
    #[error(transparent)]
    Password(#[from] PasswordParseError),
}

impl TryFrom<json_api::Credentials> for Credentials {
    type Error = CredentialParsingError;
    fn try_from(
        json_api::Credentials { email, password }: json_api::Credentials,
    ) -> Result<Self, Self::Error> {
        let email = email.parse::<EmailAddress>()?;
        let password = password.parse::<Password>()?;
        Ok(Self { email, password })
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (code, message) = match self {
            Self::CreateAccount(err) => match err {
                usecases::CreateAccountError::AlreadyExists => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                usecases::CreateAccountError::Repo(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            Self::CreateAccountEmail(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::CreateAccountPassword(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::Login(err) => match err {
                usecases::LoginError::Credentials => (StatusCode::BAD_REQUEST, err.to_string()),
                usecases::LoginError::Repo(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
            Self::LoginEmail(_) | Self::LoginPassword(_) => (
                StatusCode::BAD_REQUEST,
                "invalid email or password".to_string(),
            ),
            Self::Logout(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::Auth(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            Self::EmailNonce(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::ConfirmEmail(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            Self::ResetPassword(err) => match err {
                usecases::ResetPasswordError::InvalidToken
                | usecases::ResetPasswordError::NotFound => {
                    (StatusCode::BAD_REQUEST, err.to_string())
                }
                usecases::ResetPasswordError::Repo(err) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                }
            },
        };
        let err = json_api::Error { message };
        (code, Json(err)).into_response()
    }
}
