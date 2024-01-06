use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use klick_application::UserRecord;
use klick_domain::EmailAddress;

use crate::user::{models, schema};

pub fn fetch_user_from_db(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<Option<UserRecord>> {
    use schema::users::dsl;

    let results = dsl::users
        .filter(dsl::email.eq(email.as_str()))
        .select(models::User::as_select())
        .load(conn);

    let results = match results {
        Ok(results) => results,
        Err(diesel::result::Error::NotFound) => return Ok(None),
        Err(err) => return Err(err.into()),
    };
    debug_assert!(results.len() <= 1);
    let Some(user) = results.into_iter().next() else {
        return Ok(None);
    };
    let user = UserRecord::try_from(user).expect("Valid user record");
    Ok(Some(user))
}

pub fn insert_or_update_user(
    conn: &mut SqliteConnection,
    user: models::SaveUser<'_>,
) -> anyhow::Result<()> {
    use schema::users::dsl;

    diesel::insert_into(dsl::users)
        .values(user.clone())
        .on_conflict(dsl::email)
        .do_update()
        .set(user)
        .execute(conn)?;
    Ok(())
}

pub fn delete_user_from_db(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<()> {
    use schema::users::dsl;
    diesel::delete(dsl::users)
        .filter(dsl::email.eq(email.as_str()))
        .execute(conn)?;
    Ok(())
}
