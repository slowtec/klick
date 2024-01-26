use diesel::{prelude::*, sqlite::SqliteConnection};
use time::OffsetDateTime;

use klick_domain::{AccountToken, EmailAddress, EmailNonce};

use crate::{account, account_token::models, schema};

pub fn replace_account_token(
    conn: &mut SqliteConnection,
    token: AccountToken,
) -> anyhow::Result<EmailNonce> {
    use schema::account_tokens::dsl;

    let account_rowid =
        account::queries::resolve_account_rowid_created_by_email(conn, &token.email_nonce.email)?;
    let nonce = &token.email_nonce.nonce.to_string();
    let expires_at = token.expires_at.unix_timestamp();
    let model = models::NewAccountToken {
        account_rowid,
        nonce,
        expires_at,
    };

    diesel::insert_into(dsl::account_tokens)
        .values(model.clone())
        .on_conflict(dsl::account_rowid)
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
    let account_rowid_subselect = a_dsl::accounts
        .select(a_dsl::rowid)
        .filter(a_dsl::email.eq(email_nonce.email.as_str()));

    let target = t_dsl::account_tokens
        .filter(t_dsl::nonce.eq(email_nonce.nonce.to_string()))
        .filter(t_dsl::account_rowid.eq_any(account_rowid_subselect));

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
        .select((a_dsl::rowid, t_dsl::nonce, t_dsl::expires_at, a_dsl::email))
        .filter(a_dsl::email.eq(email.as_str()))
        .first::<models::AccountToken>(conn)?;
    AccountToken::try_from(token)
}
