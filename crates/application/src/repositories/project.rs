use klick_domain::{EmailAddress, Project, ProjectId};

pub trait Repo {
    fn find_project(&self, id: &ProjectId) -> anyhow::Result<Option<Project>>;
    fn all_projects_by_owner(&self, owner: &EmailAddress) -> anyhow::Result<Vec<Project>>;
    fn save_project(&self, project: Project, owner: &EmailAddress) -> anyhow::Result<()>;
    fn delete_project(&self, id: &ProjectId) -> anyhow::Result<()>;
}
