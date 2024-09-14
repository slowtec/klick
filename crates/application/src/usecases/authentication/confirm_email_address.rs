use thiserror::Error;

use klick_domain::EmailNonce;
use klick_interfaces::{AccountRepo, AccountTokenRepo};

use crate::usecases;

pub fn confirm_email_address<R>(repo: R, email_nonce: &EmailNonce) -> Result<(), Error>
where
    R: AccountRepo + AccountTokenRepo,
{
    // The token should be consumed only once, even if the following fails!
    let token = usecases::consume_account_token(&repo, email_nonce).map_err(|err| {
        log::warn!(
            "missing or invalid token to reset password for account '{:?}': {err}",
            email_nonce.email,
        );
        Error::InvalidToken
    })?;

    // The consumed nonce must match the request parameters
    debug_assert!(token.email_nonce == *email_nonce);

    log::info!("Confirming address of email ({})", email_nonce.email);
    let Some(mut record) = repo.find_account(&email_nonce.email)? else {
        return Err(Error::NotFound);
    };
    if !record.account.email_confirmed {
        record.account.email_confirmed = true;
        repo.save_account(&record)?;
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing or invalid token")]
    InvalidToken,
    #[error("no such account")]
    NotFound,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
