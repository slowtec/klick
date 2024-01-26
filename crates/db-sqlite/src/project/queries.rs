use diesel::{prelude::*, sqlite::SqliteConnection};

use klick_boundary as boundary;
use klick_domain::{self as domain, EmailAddress, ProjectId};

type Project = domain::Project<boundary::ProjectData>;

use crate::{account, project::models, schema};

pub fn find_project(
    conn: &mut SqliteConnection,
    id: &ProjectId,
) -> Result<Option<Project>, anyhow::Error> {
    use schema::projects::dsl;

    let id = id.to_string();
    let results = dsl::projects
        .filter(dsl::project_id.eq(&id))
        .select(models::ProjectQuery::as_select())
        .load(conn);

    let results = match results {
        Ok(results) => results,
        Err(diesel::result::Error::NotFound) => return Ok(None),
        Err(err) => return Err(err.into()),
    };
    debug_assert!(results.len() <= 1);
    let Some(record) = results.into_iter().next() else {
        return Ok(None);
    };
    let project = Project::try_from(record).expect("Valid project record");
    Ok(Some(project))
}

pub fn all_projects_by_owner(
    conn: &mut SqliteConnection,
    owner: &EmailAddress,
) -> Result<Vec<Project>, anyhow::Error> {
    use schema::projects::dsl;

    let account_rowid = account::queries::resolve_account_rowid_created_by_email(conn, owner)?;
    let results = dsl::projects
        .filter(dsl::account_rowid.eq(account_rowid))
        .select(models::ProjectQuery::as_select())
        .load(conn);
    let results = match results {
        Ok(results) => results,
        Err(diesel::result::Error::NotFound) => return Ok(vec![]),
        Err(err) => return Err(err.into()),
    };
    let projects = results
        .into_iter()
        .map(|p| Project::try_from(p).expect("Valid project record"))
        .collect();
    Ok(projects)
}

pub fn save_project(
    conn: &mut SqliteConnection,
    project: Project,
    owner: &EmailAddress,
) -> Result<(), anyhow::Error> {
    use schema::projects::dsl;

    let project_id = project.id.to_string();
    let data = models::project_to_json_string(project)?;
    let account_rowid = account::queries::resolve_account_rowid_created_by_email(conn, owner)?;

    let changeset = models::ProjectChangeset {
        account_rowid,
        project_id: &project_id,
        data: &data,
    };
    diesel::insert_into(dsl::projects)
        .values(changeset.clone())
        .on_conflict(dsl::project_id)
        .do_update()
        .set(changeset)
        .execute(conn)?;
    Ok(())
}

pub fn delete_project(conn: &mut SqliteConnection, id: &ProjectId) -> Result<(), anyhow::Error> {
    use schema::projects::dsl;

    let id = id.to_string();

    diesel::delete(dsl::projects)
        .filter(dsl::project_id.eq(&id))
        .execute(conn)?;
    Ok(())
}
