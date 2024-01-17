use thiserror::Error;

use klick_domain::{Account, EmailAddress, Password};

use crate::{usecases, AccountRecord, AccountRepo, AccountTokenRepo, NotificationGateway};

pub fn create_account<R, N>(
    repo: &R,
    notification_gateway: &N,
    email_address: EmailAddress,
    password: &Password,
) -> Result<(), Error>
where
    R: AccountRepo + AccountTokenRepo,
    N: NotificationGateway,
{
    if repo.find_account(&email_address)?.is_some() {
        return Err(Error::AlreadyExists);
    };
    let account = Account {
        email: email_address.clone(),
        email_confirmed: false,
    };
    let password = password.to_hashed();
    let record = AccountRecord { account, password };
    repo.save_account(&record)?;
    usecases::send_confirmation_email(repo, notification_gateway, email_address)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("account already exists")]
    AlreadyExists,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
