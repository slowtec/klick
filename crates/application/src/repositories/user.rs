use klick_domain::{EmailAddress, HashedPassword, User};

pub trait Repo {
    fn find_user(&self, email: &EmailAddress) -> anyhow::Result<Option<Record>>;
    fn save_user(&self, record: &Record) -> anyhow::Result<()>;
    fn delete_user(&self, email: &EmailAddress) -> anyhow::Result<()>;
}

// NOTE:
// We don't want to tie the password directly
// to the user entity, but we want to persist it
// currently at the same place, therefore we use
// a wrapper here.
#[derive(Debug)]
pub struct Record {
    pub user: User,
    pub password: HashedPassword,
}
