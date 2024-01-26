use anyhow::anyhow;

use klick_domain::{Project, ProjectId};

use crate::ProjectRepo;

pub fn read_project<R, D>(repo: &R, id: ProjectId) -> anyhow::Result<Project<D>>
where
    R: ProjectRepo<D>,
{
    let Some(project) = repo.find_project(&id)? else {
        return Err(anyhow!("project not found"));
    };
    Ok(project)
}
