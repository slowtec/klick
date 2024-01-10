use thiserror::Error;

use klick_domain::{Account, EmailAddress, EmailNonce, Nonce, Password};

use crate::{AccountRecord, AccountRepo, NotificationEvent, NotificationGateway};

pub fn create_account<R, N>(
    repo: R,
    notification_gateway: N,
    email: EmailAddress,
    password: &Password,
) -> Result<(), Error>
where
    R: AccountRepo,
    N: NotificationGateway,
{
    if repo.find_account(&email)?.is_some() {
        return Err(Error::AlreadyExists);
    };
    let account = Account {
        email: email.clone(),
        email_confirmed: false,
    };
    let password = password.to_hashed();
    let record = AccountRecord { account, password };
    repo.save_account(&record)?;
    let token = EmailNonce {
        email: email.clone(),
        nonce: Nonce::new(),
    };
    let event = NotificationEvent::AccountWasCreated(email, token);
    notification_gateway.notify(event);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("account already exists")]
    AlreadyExists,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
