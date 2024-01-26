use klick_domain::{EmailAddress, Password};

use crate::{usecases, AccountRepo, AccountTokenRepo, NotificationGateway};

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
    match usecases::login(repo, &email, password) {
        Ok(_) => {
            log::warn!("Unexpected state: user already confirmed his email address ({email})");
            return Ok(());
        }
        Err(usecases::LoginError::Credentials) => {
            // ignore this request
            return Ok(());
        }
        Err(usecases::LoginError::Repo(err)) => {
            return Err(err);
        }
        Err(usecases::LoginError::EmailNotConfirmed) => {
            // yes, it's a valid request now
        }
    }
    usecases::send_confirmation_email(repo, notification_gateway, email)?;
    Ok(())
}
