use diesel::prelude::*;

use klick_boundary as boundary;
use klick_domain as domain;

use crate::{project::models, schema};

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Project {
    pub project_id: String,
    pub data: String,
}

#[derive(Debug, Clone, AsChangeset, Insertable)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProjectChangeset<'a> {
    pub project_id: &'a str,
    pub account_rowid: i64,
    pub data: &'a str,
}

impl TryFrom<Project> for domain::Project {
    type Error = anyhow::Error;

    fn try_from(from: Project) -> Result<Self, Self::Error> {
        let models::Project { project_id, data } = from;
        let project = project_from_json_str(&data).expect("valid project json data");
        debug_assert_eq!(project_id.parse::<domain::ProjectId>().unwrap(), project.id);
        Ok(project)
    }
}

pub fn project_to_json_string(project: domain::Project) -> String {
    let saved_project = boundary::SavedProject::from(project);
    let project = boundary::Project::from(saved_project);
    let data = boundary::Data { project };
    boundary::export_to_string(&data)
}

pub fn project_from_json_str(json: &str) -> anyhow::Result<domain::Project> {
    let project = boundary::import_from_str(json)?;
    domain::Project::try_from(project)
}
