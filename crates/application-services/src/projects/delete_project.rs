use klick_domain::ProjectId;
use klick_interfaces::ProjectRepo;

pub fn delete_project<R, D>(repo: &R, id: ProjectId) -> anyhow::Result<()>
where
    R: ProjectRepo<D>,
{
    repo.delete_project(&id)
}
