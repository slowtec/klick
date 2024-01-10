use thiserror::Error;

use klick_domain::EmailNonce;

use crate::AccountRepo;

pub fn confirm_email_address<R>(repo: R, email_nonce: &EmailNonce) -> Result<(), Error>
where
    R: AccountRepo,
{
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
    #[error("no such account")]
    NotFound,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
