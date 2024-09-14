use klick_domain::{EmailAddress, Password};
use klick_interfaces::{AccountRepo, AccountTokenRepo, NotificationGateway};

use crate::{login, send_confirmation_email, LoginError};

pub fn resend_confirmation_email<R, N>(
    repo: &R,
    notification_gateway: &N,
    email: EmailAddress,
    password: &Password,
) -> anyhow::Result<()>
where
    R: AccountRepo + AccountTokenRepo,
    N: NotificationGateway,
{
    match login(repo, &email, password) {
        Ok(_) => {
            log::warn!("Unexpected state: user already confirmed his email address ({email})");
            return Ok(());
        }
        Err(LoginError::Credentials) => {
            // ignore this request
            return Ok(());
        }
        Err(LoginError::Repo(err)) => {
            return Err(err);
        }
        Err(LoginError::EmailNotConfirmed) => {
            // yes, it's a valid request now
        }
    }
    send_confirmation_email(repo, notification_gateway, email)?;
    Ok(())
}
