use std::fmt;

use gloo_net::http::{Request, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use thiserror::Error;

use klick_boundary::json_api::{
    self, ApiToken, ConfirmEmailAddress, Credentials, RequestPasswordReset, ResetPassword, UserInfo,
};

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone)]
pub struct AuthorizedApi {
    url: &'static str,
    token: ApiToken,
}

impl fmt::Debug for AuthorizedApi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AuthorizedApi")
            .field("url", &self.url)
            .field("token", &"***")
            .finish()
    }
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn register(&self, credentials: &Credentials) -> Result<(), Value> {
        let url = format!("{}/users", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        into_json(response).await
    }

    pub async fn login(
        &self,
        credentials: &Credentials,
    ) -> Result<AuthorizedApi, json_api::login::Error> {
        log::debug!("Try to login ({})", credentials.email);
        let url = format!("{}/login", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        let token = into_json(response).await?;
        Ok(AuthorizedApi::new(self.url, token))
    }

    pub async fn resend_confirmation_mail(&self, credentials: &Credentials) -> Result<(), Value> {
        log::debug!(
            "Request new email to confirm email address for ({})",
            credentials.email
        );
        let url = format!("{}/users/resent-confirmation-email", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        into_json(response).await
    }

    pub async fn request_password_reset(&self, email: String) -> Result<(), Value> {
        let url = format!("{}/users/reset-password-request", self.url);
        let response = Request::post(&url)
            .json(&RequestPasswordReset { email })?
            .send()
            .await?;
        into_json(response).await
    }

    pub async fn reset_password(&self, token: String, new_password: String) -> Result<(), Value> {
        let url = format!("{}/users/reset-password", self.url);
        let response = Request::post(&url)
            .json(&ResetPassword {
                token,
                new_password,
            })?
            .send()
            .await?;
        into_json(response).await
    }

    pub async fn confirm_email_address(&self, token: String) -> Result<(), Value> {
        let url = format!("{}/users/confirm-email-address", self.url);
        let response = Request::post(&url)
            .json(&ConfirmEmailAddress { token })?
            .send()
            .await?;
        into_json(response).await
    }
}

impl AuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }
    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }
    async fn send<T>(&self, req: RequestBuilder) -> Result<T, Value>
    where
        T: DeserializeOwned,
    {
        let response = req
            .header("Authorization", &self.auth_header_value())
            .send()
            .await?;
        into_json(response).await
    }
    pub async fn logout(&self) -> Result<(), Value> {
        let url = format!("{}/logout", self.url);
        self.send(Request::post(&url)).await
    }
    pub async fn user_info(&self) -> Result<UserInfo, Value> {
        let url = format!("{}/users", self.url);
        self.send(Request::get(&url)).await
    }
    pub fn token(&self) -> &ApiToken {
        &self.token
    }
}

type Result<T, E> = std::result::Result<T, Error<E>>;

#[derive(Debug, Error)]
pub enum Error<T> {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(json_api::Error<T>),
}

impl<T> From<json_api::Error<T>> for Error<T> {
    fn from(e: json_api::Error<T>) -> Self {
        Self::Api(e)
    }
}

async fn into_json<T, E>(response: Response) -> Result<T, E>
where
    T: DeserializeOwned,
    E: DeserializeOwned,
    Error<E>: From<json_api::Error<E>>,
{
    // ensure we've got 2xx status
    if response.ok() {
        Ok(response.json().await?)
    } else {
        log::warn!("Response status: {}", response.status());
        Err(response.json::<json_api::Error<E>>().await?.into())
    }
}
