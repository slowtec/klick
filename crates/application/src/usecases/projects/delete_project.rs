use klick_domain::ProjectId;

use crate::ProjectRepo;

pub fn delete_project<R, D>(repo: &R, id: ProjectId) -> anyhow::Result<()>
where
    R: ProjectRepo<D>,
{
    repo.delete_project(&id)
}
