use diesel::prelude::*;
use time::OffsetDateTime;

use klick_application as app;
use klick_domain::{Account, EmailAddress, HashedPassword};

use crate::schema;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AccountQuery {
    pub email: String,
    pub email_confirmed: bool,
    pub password: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AccountChangeset<'a> {
    pub email: &'a str,
    pub email_confirmed: bool,
    pub password: &'a str,
    pub created_at: i64,
}

impl TryFrom<AccountQuery> for app::AccountRecord {
    type Error = anyhow::Error;

    fn try_from(from: AccountQuery) -> Result<Self, Self::Error> {
        let AccountQuery {
            email,
            email_confirmed,
            password,
            created_at,
        } = from;
        let email_address = email.parse::<EmailAddress>()?;
        let created_at = OffsetDateTime::from_unix_timestamp(created_at)?;
        let account = Account {
            email_address,
            email_confirmed,
            created_at,
        };
        let password = HashedPassword::from_hash(password);
        let record = Self { account, password };
        Ok(record)
    }
}

impl<'a> TryFrom<&'a app::AccountRecord> for AccountChangeset<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a app::AccountRecord) -> Result<Self, Self::Error> {
        let app::AccountRecord { account, password } = record;
        let Account {
            email_address,
            email_confirmed,
            created_at,
        } = account;
        let created_at = created_at.unix_timestamp();
        let email = email_address.as_str();
        let password = password.as_str();
        let email_confirmed = *email_confirmed;
        let account = Self {
            email,
            email_confirmed,
            password,
            created_at,
        };
        Ok(account)
    }
}
