use klick_domain::authentication::{Account, EmailAddress, HashedPassword};

pub trait Repo {
    fn find_account(&self, email: &EmailAddress) -> anyhow::Result<Option<Record>>;
    fn save_account(&self, record: &Record) -> anyhow::Result<()>;
    fn delete_account(&self, email: &EmailAddress) -> anyhow::Result<()>;
}

// NOTE:
// We don't want to tie the password directly
// to the user entity, but we want to persist it
// currently at the same place, therefore we use
// a wrapper here.
#[derive(Debug)]
pub struct Record {
    pub account: Account,
    pub password: HashedPassword,
}
