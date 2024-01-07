use diesel::prelude::*;

use klick_application as app;
use klick_domain as domain;

use crate::account::{models, schema};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub email: String,
    pub email_confirmed: bool,
    pub password: String,
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name = schema::accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SaveAccount<'a> {
    pub email: &'a str,
    pub email_confirmed: bool,
    pub password: &'a str,
}

impl TryFrom<Account> for app::AccountRecord {
    type Error = anyhow::Error;

    fn try_from(from: Account) -> Result<Self, Self::Error> {
        let models::Account {
            email,
            email_confirmed,
            password,
        } = from;
        let email = email.parse::<domain::EmailAddress>()?;
        let account = domain::Account {
            email,
            email_confirmed,
        };
        let password = domain::HashedPassword::from_hash(password);
        let record = app::AccountRecord { account, password };
        Ok(record)
    }
}

impl<'a> TryFrom<&'a app::AccountRecord> for SaveAccount<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a app::AccountRecord) -> Result<Self, Self::Error> {
        let app::AccountRecord { account, password } = record;
        let domain::Account {
            email,
            email_confirmed,
        } = account;
        let account = models::SaveAccount {
            email: email.as_str(),
            email_confirmed: *email_confirmed,
            password: password.as_str(),
        };
        Ok(account)
    }
}
