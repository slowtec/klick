use thiserror::Error;

use klick_domain::authentication::{EmailNonce, HashedPassword};

use crate::{usecases, AccountRepo, AccountTokenRepo};

pub fn reset_password<R>(
    repo: R,
    email_nonce: EmailNonce,
    new_password: HashedPassword,
) -> Result<(), Error>
where
    R: AccountRepo + AccountTokenRepo,
{
    // The token should be consumed only once, even if the following fails!
    let token = usecases::consume_account_token(&repo, &email_nonce).map_err(|err| {
        log::warn!(
            "missing or invalid token to reset password for account '{:?}': {err}",
            email_nonce.email,
        );
        Error::InvalidToken
    })?;

    // The consumed nonce must match the request parameters
    debug_assert!(token.email_nonce == email_nonce);

    // Verify and update the account entity
    log::info!("Resetting password for email ({})", email_nonce.email);
    let Some(mut record) = repo.find_account(&email_nonce.email)? else {
        return Err(Error::NotFound);
    };
    debug_assert_eq!(record.account.email_address, email_nonce.email);
    record.account.email_confirmed = true;
    record.password = new_password;
    repo.save_account(&record)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("missing or invalid token")]
    InvalidToken,
    #[error("account does not exist")]
    NotFound,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
