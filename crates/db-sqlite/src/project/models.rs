use diesel::prelude::*;

use klick_boundary as boundary;
use klick_domain as domain;

use crate::schema;

type Project = domain::Project<boundary::ProjectData>;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = schema::projects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProjectQuery {
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

impl TryFrom<ProjectQuery> for Project {
    type Error = anyhow::Error;

    fn try_from(from: ProjectQuery) -> Result<Self, Self::Error> {
        let ProjectQuery { project_id, data } = from;
        let project = project_from_json_str(&data).expect("valid project json data");
        debug_assert_eq!(project_id.parse::<domain::ProjectId>().unwrap(), project.id);
        Ok(project)
    }
}

pub fn project_to_json_string(project: Project) -> anyhow::Result<String> {
    let project = boundary::Project::try_from(project)?;
    let data = boundary::Data { project };
    let string = boundary::export_to_string(&data);
    Ok(string)
}

pub fn project_from_json_str(json: &str) -> anyhow::Result<Project> {
    let project = boundary::import_from_str(json)?;
    let project = match project {
        boundary::Project::Saved(project) => project,
        boundary::Project::Unsaved(_) => unreachable!(),
    };
    Ok(Project::try_from(project)?)
}