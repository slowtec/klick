use thiserror::Error;
use time::OffsetDateTime;

use klick_domain::{AccountToken, EmailNonce};
use klick_interfaces::AccountTokenRepo;

pub fn consume_account_token<R>(repo: &R, email_nonce: &EmailNonce) -> Result<AccountToken, Error>
where
    R: AccountTokenRepo,
{
    let token = repo.consume_account_token(email_nonce)?;
    debug_assert_eq!(email_nonce, &token.email_nonce);
    if token.expires_at < OffsetDateTime::now_utc() {
        return Err(Error::TokenExpired);
    }
    Ok(token)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("the token expired")]
    TokenExpired,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
