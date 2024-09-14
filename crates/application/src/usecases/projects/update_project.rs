use anyhow::anyhow;
use time::OffsetDateTime;

use klick_domain::{Account, ProjectId};

use crate::ProjectRepo;

pub fn update_project<R, D>(
    repo: &R,
    account: &Account,
    id: &ProjectId,
    data: D,
) -> anyhow::Result<()>
where
    R: ProjectRepo<D>,
{
    let Some(mut project) = repo.find_project(id)? else {
        return Err(anyhow!("project not found"));
    };
    project.modified_at = Some(OffsetDateTime::now_utc());
    project.data = data;
    repo.save_project(project, &account.email_address)?;
    Ok(())
}
