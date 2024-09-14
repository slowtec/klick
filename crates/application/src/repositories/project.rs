use klick_domain::{EmailAddress, Project, ProjectId};

pub trait Repo<T> {
    fn find_project(&self, id: &ProjectId) -> anyhow::Result<Option<Project<T>>>;
    fn all_projects_by_owner(&self, owner: &EmailAddress) -> anyhow::Result<Vec<Project<T>>>;
    fn save_project(&self, project: Project<T>, owner: &EmailAddress) -> anyhow::Result<()>;
    fn delete_project(&self, id: &ProjectId) -> anyhow::Result<()>;
}
