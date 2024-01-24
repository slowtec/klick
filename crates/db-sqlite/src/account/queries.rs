use diesel::{prelude::*, sqlite::SqliteConnection};
use time::OffsetDateTime;

use klick_application::AccountRecord;
use klick_domain::EmailAddress;

use crate::{account::models, schema};

pub fn fetch_account_from_db(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<Option<AccountRecord>> {
    use schema::accounts::dsl;

    let results = dsl::accounts
        .filter(dsl::email.eq(email.as_str()))
        .select(models::Account::as_select())
        .load(conn);

    let results = match results {
        Ok(results) => results,
        Err(diesel::result::Error::NotFound) => return Ok(None),
        Err(err) => return Err(err.into()),
    };
    debug_assert!(results.len() <= 1);
    let Some(account) = results.into_iter().next() else {
        return Ok(None);
    };
    let account = AccountRecord::try_from(account).expect("Valid account record");
    Ok(Some(account))
}

pub fn insert_or_update_account(
    conn: &mut SqliteConnection,
    account: models::AccountChangeset<'_>,
) -> anyhow::Result<()> {
    use schema::accounts::dsl;

    diesel::insert_into(dsl::accounts)
        .values(account.clone())
        .on_conflict(dsl::email)
        .do_update()
        .set(account)
        .execute(conn)?;
    Ok(())
}

pub fn delete_account_from_db(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<()> {
    use schema::accounts::dsl;

    diesel::delete(dsl::accounts)
        .filter(dsl::email.eq(email.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn resolve_account_rowid_created_by_email(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<i64> {
    use schema::accounts::dsl;

    let id = dsl::accounts
        .select(dsl::rowid)
        .filter(dsl::email.eq(email.as_str()))
        .first(conn)?;
    Ok(id)
}

pub fn delete_unconfirmed_accounts(
    conn: &mut SqliteConnection,
    created_before: OffsetDateTime,
) -> anyhow::Result<()> {
    use schema::accounts::dsl;

    let created_before = created_before.unix_timestamp();
    diesel::delete(dsl::accounts)
        .filter(
            dsl::email_confirmed
                .eq(false)
                .and(dsl::created_at.le(created_before)),
        )
        .execute(conn)?;
    Ok(())
}
