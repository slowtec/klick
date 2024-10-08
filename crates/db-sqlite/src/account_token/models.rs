use diesel::prelude::*;
use time::OffsetDateTime;

use klick_domain::{AccountToken, EmailAddress, EmailNonce, Nonce};

use crate::schema;

#[derive(Queryable)]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AccountTokenQuery {
    pub account_rowid: i64,
    pub nonce: String,
    pub expires_at: i64,
    // Joined columns
    pub account_email: String,
}

#[derive(Clone, Insertable, AsChangeset)]
#[diesel(table_name = schema::account_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewAccountToken<'a> {
    pub account_rowid: i64,
    pub nonce: &'a str,
    pub expires_at: i64,
}

impl TryFrom<AccountTokenQuery> for AccountToken {
    type Error = anyhow::Error;
    fn try_from(from: AccountTokenQuery) -> Result<Self, Self::Error> {
        let AccountTokenQuery {
            account_rowid: _,
            nonce,
            expires_at,
            account_email,
        } = from;

        let email = EmailAddress::new_unchecked(account_email);
        let nonce = nonce.parse::<Nonce>()?;
        let email_nonce = EmailNonce { email, nonce };
        let expires_at = OffsetDateTime::from_unix_timestamp(expires_at)?;

        Ok(Self {
            email_nonce,
            expires_at,
        })
    }
}
