use gloo_net::http::{Request, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;

use klick_boundary::json_api::{
    self, ApiToken, ConfirmEmailAddress, Credentials, RequestPasswordReset, UserInfo,
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

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn register(&self, credentials: &Credentials) -> Result<()> {
        let url = format!("{}/users", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        into_json(response).await
    }

    pub async fn login(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/login", self.url);
        let response = Request::post(&url).json(credentials)?.send().await?;
        let token = into_json(response).await?;
        Ok(AuthorizedApi::new(self.url, token))
    }

    pub async fn request_password_reset(&self, email: String) -> Result<()> {
        let url = format!("{}/users/reset-password-request", self.url);
        let response = Request::post(&url)
            .json(&RequestPasswordReset { email })?
            .send()
            .await?;
        into_json(response).await
    }

    pub async fn confirm_email_address(&self, token: String) -> Result<()> {
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
    async fn send<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = req
            .header("Authorization", &self.auth_header_value())
            .send()
            .await?;
        into_json(response).await
    }
    pub async fn logout(&self) -> Result<()> {
        let url = format!("{}/logout", self.url);
        self.send(Request::post(&url)).await
    }
    pub async fn user_info(&self) -> Result<UserInfo> {
        let url = format!("{}/users", self.url);
        self.send(Request::get(&url)).await
    }
    pub fn token(&self) -> &ApiToken {
        &self.token
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(json_api::Error),
}

impl From<json_api::Error> for Error {
    fn from(e: json_api::Error) -> Self {
        Self::Api(e)
    }
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(response.json::<json_api::Error>().await?.into())
    }
}
