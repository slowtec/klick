use thiserror::Error;
use time::OffsetDateTime;

use klick_domain::{Account, EmailAddress, Password};
use klick_interfaces::{AccountRecord, AccountRepo, AccountTokenRepo, NotificationGateway};

use crate::send_confirmation_email;

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
    if let Some(AccountRecord { account, .. }) = repo.find_account(&email_address)? {
        return if account.email_confirmed {
            Err(Error::AlreadyExists)
        } else {
            Ok(send_confirmation_email(
                repo,
                notification_gateway,
                email_address,
            )?)
        };
    };
    let created_at = OffsetDateTime::now_utc();
    let account = Account {
        email_address: email_address.clone(),
        email_confirmed: false,
        created_at,
    };
    let password = password.to_hashed();
    let record = AccountRecord { account, password };
    repo.save_account(&record)?;
    send_confirmation_email(repo, notification_gateway, email_address)?;
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("account already exists")]
    AlreadyExists,
    #[error(transparent)]
    Repo(#[from] anyhow::Error),
}
