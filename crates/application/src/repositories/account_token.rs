use time::OffsetDateTime;

use klick_domain::authentication::{AccountToken, EmailAddress, EmailNonce};

pub trait Repo {
    fn replace_account_token(&self, account_token: AccountToken) -> anyhow::Result<EmailNonce>;
    fn consume_account_token(&self, email_nonce: &EmailNonce) -> anyhow::Result<AccountToken>;
    fn delete_expired_account_tokens(
        &self,
        expired_before: OffsetDateTime,
    ) -> anyhow::Result<usize>;
    fn get_account_token_by_email(&self, email: &EmailAddress) -> anyhow::Result<AccountToken>;
}
