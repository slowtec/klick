use diesel::{prelude::*, sqlite::SqliteConnection};
use time::OffsetDateTime;

use klick_domain::{AccountToken, EmailAddress, EmailNonce};

use crate::{account_token::models, schema};

pub fn replace_account_token(
    conn: &mut SqliteConnection,
    token: AccountToken,
) -> anyhow::Result<EmailNonce> {
    use schema::account_tokens::dsl;

    let account_id = resolve_account_created_by_email(conn, &token.email_nonce.email)?;
    let nonce = &token.email_nonce.nonce.to_string();
    let expires_at = token.expires_at.unix_timestamp();
    let model = models::NewAccountToken {
        account_id,
        nonce,
        expires_at,
    };

    diesel::insert_into(dsl::account_tokens)
        .values(model.clone())
        .on_conflict(dsl::account_id)
        .do_update()
        .set(model)
        .execute(conn)?;
    Ok(token.email_nonce)
}

pub fn consume_account_token(
    conn: &mut SqliteConnection,
    email_nonce: &EmailNonce,
) -> anyhow::Result<AccountToken> {
    use schema::{account_tokens::dsl as t_dsl, accounts::dsl as a_dsl};

    let token = account_token_by_email(conn, &email_nonce.email)?;
    let account_id_subselect = a_dsl::accounts
        .select(a_dsl::id)
        .filter(a_dsl::email.eq(email_nonce.email.as_str()));

    let target = t_dsl::account_tokens
        .filter(t_dsl::nonce.eq(email_nonce.nonce.to_string()))
        .filter(t_dsl::account_id.eq_any(account_id_subselect));

    if diesel::delete(target).execute(conn)? == 0 {
        anyhow::bail!("not found");
    }
    debug_assert_eq!(email_nonce, &token.email_nonce);
    Ok(token)
}

pub fn delete_expired_account_tokens(
    conn: &mut SqliteConnection,
    expired_before: OffsetDateTime,
) -> anyhow::Result<usize> {
    use schema::account_tokens::dsl;

    let count = diesel::delete(
        dsl::account_tokens.filter(dsl::expires_at.lt(expired_before.unix_timestamp())),
    )
    .execute(conn)?;
    Ok(count)
}

pub fn account_token_by_email(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<AccountToken> {
    use schema::{account_tokens::dsl as t_dsl, accounts::dsl as a_dsl};

    let token = t_dsl::account_tokens
        .inner_join(a_dsl::accounts)
        .select((a_dsl::id, t_dsl::nonce, t_dsl::expires_at, a_dsl::email))
        .filter(a_dsl::email.eq(email.as_str()))
        .first::<models::AccountToken>(conn)?;
    Ok(AccountToken::try_from(token)?)
}

fn resolve_account_created_by_email(
    conn: &mut SqliteConnection,
    email: &EmailAddress,
) -> anyhow::Result<i64> {
    use schema::accounts::dsl;

    let id = dsl::accounts
        .select(dsl::id)
        .filter(dsl::email.eq(email.as_str()))
        .first(conn)?;
    Ok(id)
}