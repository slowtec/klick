use diesel::prelude::*;

use klick_application as app;
use klick_domain as domain;

use crate::user::{models, schema};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub email: String,
    pub email_confirmed: bool,
    pub password: String,
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SaveUser<'a> {
    pub email: &'a str,
    pub email_confirmed: bool,
    pub password: &'a str,
}

impl TryFrom<User> for app::UserRecord {
    type Error = anyhow::Error;

    fn try_from(from: User) -> Result<Self, Self::Error> {
        let models::User {
            email,
            email_confirmed,
            password,
        } = from;
        let email = email.parse::<domain::EmailAddress>()?;
        let user = domain::User {
            email,
            email_confirmed,
        };
        let password = domain::HashedPassword::from_hash(password);
        let record = app::UserRecord { user, password };
        Ok(record)
    }
}

impl<'a> TryFrom<&'a app::UserRecord> for SaveUser<'a> {
    type Error = anyhow::Error;

    fn try_from(record: &'a app::UserRecord) -> Result<Self, Self::Error> {
        let app::UserRecord { user, password } = record;
        let domain::User {
            email,
            email_confirmed,
        } = user;
        let user = models::SaveUser {
            email: email.as_str(),
            email_confirmed: *email_confirmed,
            password: password.as_str(),
        };
        Ok(user)
    }
}
