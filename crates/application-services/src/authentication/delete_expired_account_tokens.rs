use time::OffsetDateTime;

use klick_interfaces::AccountTokenRepo;

pub fn delete_expired_account_tokens<R>(repo: &R) -> anyhow::Result<usize>
where
    R: AccountTokenRepo,
{
    let expired_before = OffsetDateTime::now_utc();
    let count = repo.delete_expired_account_tokens(expired_before)?;
    Ok(count)
}
