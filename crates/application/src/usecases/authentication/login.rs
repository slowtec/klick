use thiserror::Error;

use klick_domain::authentication::{Account, EmailAddress, Password};

use crate::AccountRepo;

pub fn login<R>(repo: &R, email: &EmailAddress, password: &Password) -> Result<Account, Error>
where
    R: AccountRepo,
{
    let Some(record) = repo.find_account(email)? else {
        return Err(Error::Credentials);
    };
    if !record.password.verify(password) {
        return Err(Error::Credentials);
    }
    if !record.account.email_confirmed {
        return Err(Error::EmailNotConfirmed);
    }
    Ok(record.account)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid email or password")]
    Credentials,
    #[error("your email is not confirmed yet")]
    EmailNotConfirmed,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
