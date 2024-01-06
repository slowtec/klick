use thiserror::Error;

use klick_domain::{EmailAddress, Password, User};

use crate::UserRepo;

pub fn login<R>(repo: R, email: &EmailAddress, password: &Password) -> Result<User, Error>
where
    R: UserRepo,
{
    let Some(record) = repo.find_user(email)? else {
        return Err(Error::Credentials);
    };
    if !record.password.verify(password) {
        return Err(Error::Credentials);
    }
    Ok(record.user)
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid email or password")]
    Credentials,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
