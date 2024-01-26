use klick_domain::{Account, Project};

use crate::ProjectRepo;

pub fn read_all_projects<R, D>(repo: &R, account: &Account) -> anyhow::Result<Vec<Project<D>>>
where
    R: ProjectRepo<D>,
{
    let projects = repo.all_projects_by_owner(&account.email_address)?;
    Ok(projects)
}
