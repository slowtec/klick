use klick_domain::EmailAddress;
use klick_interfaces::{AccountTokenRepo, NotificationEvent, NotificationGateway};

use crate::usecases;

pub fn send_confirmation_email<R, N>(
    repo: &R,
    notification_gateway: &N,
    email_address: EmailAddress,
) -> anyhow::Result<()>
where
    R: AccountTokenRepo,
    N: NotificationGateway,
{
    let email_nonce = usecases::refresh_account_token(repo, email_address)?;
    let event = NotificationEvent::AccountWasCreated { email_nonce };
    notification_gateway.notify(event);
    Ok(())
}
