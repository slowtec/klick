use time::OffsetDateTime;

use klick_domain::{Account, Project, ProjectId};

use crate::ProjectRepo;

pub fn create_new_project<R, D>(repo: &R, account: &Account, data: D) -> anyhow::Result<ProjectId>
where
    R: ProjectRepo<D>,
{
    let created_at = OffsetDateTime::now_utc();
    let modified_at = None;
    let id = ProjectId::new();
    let project = Project {
        id,
        created_at,
        modified_at,
        data,
    };
    repo.save_project(project, &account.email_address)?;
    Ok(id)
}
