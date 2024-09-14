use anyhow::bail;

use klick_domain::EmailAddress;
use klick_interfaces::{AccountRepo, AccountTokenRepo, NotificationEvent, NotificationGateway};

use crate::usecases;

pub fn request_password_reset<R, N>(
    repo: &R,
    notification_gateway: &N,
    email_address: EmailAddress,
) -> anyhow::Result<()>
where
    R: AccountRepo + AccountTokenRepo,
    N: NotificationGateway,
{
    let Some(record) = repo.find_account(&email_address)? else {
        bail!("account not found");
    };
    if !record.account.email_confirmed {
        bail!("uncofirmed email address");
    };
    let email_nonce = usecases::refresh_account_token(repo, email_address)?;
    let event = NotificationEvent::AccountResetPasswordRequested { email_nonce };
    notification_gateway.notify(event);
    Ok(())
}
