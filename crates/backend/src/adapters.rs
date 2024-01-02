use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

use klick_boundary::json_api;
//use api_boundary as json;
//use thiserror::Error;

use crate::{application::*, Error};

impl From<InvalidEmailAddress> for json_api::Error {
    fn from(_: InvalidEmailAddress) -> Self {
        Self {
            message: "Invalid email address".to_string(),
        }
    }
}

impl From<InvalidPassword> for json_api::Error {
    fn from(err: InvalidPassword) -> Self {
        let InvalidPassword::TooShort(min_len) = err;
        Self {
            message: format!("Invalid password (min. length = {min_len})"),
        }
    }
}

impl From<CreateUserError> for json_api::Error {
    fn from(err: CreateUserError) -> Self {
        let message = match err {
            CreateUserError::UserExists => "User already exits".to_string(),
        };
        Self { message }
    }
}

impl From<LoginError> for json_api::Error {
    fn from(err: LoginError) -> Self {
        let message = match err {
            LoginError::InvalidEmailOrPassword => "Invalid email or password".to_string(),
        };
        Self { message }
    }
}

impl From<LogoutError> for json_api::Error {
    fn from(err: LogoutError) -> Self {
        let message = match err {
            LogoutError::NotLoggedIn => "No user is logged in".to_string(),
        };
        Self { message }
    }
}

impl From<AuthError> for json_api::Error {
    fn from(err: AuthError) -> Self {
        let message = match err {
            AuthError::NotAuthorized => "Not authorized".to_string(),
        };
        Self { message }
    }
}

impl From<CredentialParsingError> for json_api::Error {
    fn from(err: CredentialParsingError) -> Self {
        match err {
            CredentialParsingError::EmailAddress(err) => err.into(),
            CredentialParsingError::Password(err) => err.into(),
        }
    }
}

#[derive(Debug, Error)]
pub enum CredentialParsingError {
    #[error(transparent)]
    EmailAddress(#[from] InvalidEmailAddress),
    #[error(transparent)]
    Password(#[from] InvalidPassword),
}

impl TryFrom<json_api::Credentials> for Credentials {
    type Error = CredentialParsingError;
    fn try_from(
        json_api::Credentials { email, password }: json_api::Credentials,
    ) -> Result<Self, Self::Error> {
        let email: EmailAddress = email.parse()?;
        let password = Password::try_from(password)?;
        Ok(Self { email, password })
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (code, value) = match self {
            Self::Logout(err) => (StatusCode::BAD_REQUEST, json_api::Error::from(err)),
            Self::Login(err) => (StatusCode::BAD_REQUEST, json_api::Error::from(err)),
            Self::Credentials(err) => (StatusCode::BAD_REQUEST, json_api::Error::from(err)),
            Self::CreateUser(err) => (StatusCode::BAD_REQUEST, json_api::Error::from(err)),
            Self::Auth(err) => (StatusCode::UNAUTHORIZED, json_api::Error::from(err)),
        };
        (code, Json(value)).into_response()
    }
}
