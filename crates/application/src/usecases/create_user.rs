use thiserror::Error;

use klick_domain::{EmailAddress, Password, User};

use crate::{UserRecord, UserRepo};

pub fn create_user<R>(repo: R, email: EmailAddress, password: &Password) -> Result<(), Error>
where
    R: UserRepo,
{
    if repo.find_user(&email)?.is_some() {
        return Err(Error::AlreadyExists);
    };
    let user = User {
        email,
        email_confirmed: false,
    };
    let password = password.to_hashed();
    let record = UserRecord { user, password };
    repo.save_user(&record)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("user already exists")]
    AlreadyExists,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
