use time::{Duration, OffsetDateTime};

use klick_domain::{AccountToken, EmailAddress, EmailNonce, Nonce};

use crate::AccountTokenRepo;

const DEFAULT_EXPIRY_DURATION: Duration = Duration::days(1);

pub fn refresh_account_token<R>(repo: &R, email_address: EmailAddress) -> anyhow::Result<EmailNonce>
where
    R: AccountTokenRepo,
{
    let email_nonce = EmailNonce {
        email: email_address,
        nonce: Nonce::new(),
    };

    let expires_at = OffsetDateTime::now_utc() + DEFAULT_EXPIRY_DURATION;

    let token = AccountToken {
        email_nonce,
        expires_at,
    };
    let email_nonce = repo.replace_account_token(token)?;
    Ok(email_nonce)
}
