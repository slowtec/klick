use thiserror::Error;

use klick_domain::{Account, EmailAddress, EmailNonce, Nonce, Password};

use crate::{AccountRecord, AccountRepo, NotificationEvent, NotificationGateway};

pub fn create_account<R, N>(
    repo: R,
    notification_gateway: N,
    email_address: EmailAddress,
    password: &Password,
) -> Result<(), Error>
where
    R: AccountRepo,
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
    let email_nonce = EmailNonce {
        email: email_address,
        nonce: Nonce::new(),
    };
    let event = NotificationEvent::AccountWasCreated { email_nonce };
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
