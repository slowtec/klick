use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
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
    #[error("internal server error")]
    InternalServerError,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
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
    #[error("your email is not confirmed yet")]
    EmailNotConfirmed,
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

// TODO: tidy up and uwe json_api::Error directly
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        use json_api::Error as E;
        match self {
            Self::CreateAccount(err) => match err {
                usecases::CreateAccountError::AlreadyExists => {
                    let message = Some(err.to_string());
                    let status = StatusCode::BAD_REQUEST;
                    let details = Some(json_api::register::Error::AlreadyExists);
                    json_api::Error {
                        message,
                        status,
                        details,
                    }
                    .into_response()
                }
                usecases::CreateAccountError::Repo(_) => internal(),
            },
            Self::CreateAccountEmail(err) => bad_request(err),
            Self::CreateAccountPassword(err) => bad_request(err),
            Self::Login(err) => match err {
                usecases::LoginError::Credentials => E::bad_request()
                    .details(json_api::login::Error::Credentials)
                    .message(err)
                    .into_response(),
                usecases::LoginError::EmailNotConfirmed => E::unauthorized()
                    .details(json_api::login::Error::EmailNotConfirmed)
                    .message(err)
                    .into_response(),
                usecases::LoginError::Repo(_) => internal(),
            },
            Self::LoginEmail(_) | Self::LoginPassword(_) => E::<()>::bad_request()
                .message("invalid email or password")
                .into_response(),
            Self::Logout(err) => bad_request(err),
            Self::Auth(_) => E::<()>::unauthorized().into_response(),
            Self::EmailNonce(err) => bad_request(err).into_response(),
            Self::ConfirmEmail(err) => bad_request(err).into_response(),
            Self::ResetPassword(err) => match err {
                usecases::ResetPasswordError::InvalidToken
                | usecases::ResetPasswordError::NotFound => bad_request(err),
                usecases::ResetPasswordError::Repo(_) => internal(),
            },
            Self::InternalServerError => internal(),
            Self::Other(err) => bad_request(err),
        }
    }
}

fn internal() -> Response {
    json_api::Error::<()>::internal().into_response()
}

fn bad_request<S>(msg: S) -> Response
where
    S: ToString,
{
    json_api::Error::<()>::bad_request()
        .message(msg)
        .into_response()
}
