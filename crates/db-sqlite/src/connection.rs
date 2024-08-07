use std::sync::Arc;

use anyhow::anyhow;
use diesel::{sqlite::SqliteConnection, Connection as _};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use parking_lot::Mutex;
use thiserror::Error;
use time::OffsetDateTime;

use klick_application::{AccountRecord, AccountRepo, AccountTokenRepo, ProjectRepo};
use klick_boundary as boundary;
use klick_domain::{
    self as domain,
    authentication::{AccountToken, EmailAddress, EmailNonce},
    ProjectId,
};

use crate::{account, account_token, project};

type Project = domain::Project<boundary::JsonFormData>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Debug, Error)]
#[error("unable to connect to database: {0}")]
pub struct ConnectionError(pub(crate) String);

impl From<anyhow::Error> for ConnectionError {
    fn from(from: anyhow::Error) -> Self {
        Self(from.to_string())
    }
}

// TODO:
// Does it make sense for SQLite to replace the mutex
// with something like a connection pool?
// Or is there a more elegant way to model this connection?
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Connection(Arc<Mutex<SqliteConnection>>);

impl Connection {
    pub fn establish(db_url: &str) -> Result<Self, ConnectionError> {
        establish_connection(db_url)
    }

    pub fn run_embedded_database_migrations(&self) -> anyhow::Result<()> {
        run_embedded_database_migrations(self, MIGRATIONS)
    }
    pub fn delete_old_unconfirmed_accounts(
        &self,
        created_before: OffsetDateTime,
    ) -> anyhow::Result<()> {
        delete_old_unconfirmed_accounts(self, created_before)
    }
}

fn establish_connection(db_url: &str) -> Result<Connection, ConnectionError> {
    let sqlite_connection =
        SqliteConnection::establish(db_url).map_err(|err| ConnectionError(err.to_string()))?;
    Ok(Connection(Arc::new(Mutex::new(sqlite_connection))))
}

fn run_embedded_database_migrations(
    connection: &Connection,
    migrations: EmbeddedMigrations,
) -> anyhow::Result<()> {
    log::info!("Running embedded account database migrations");
    connection
        .0
        .lock()
        .run_pending_migrations(migrations)
        .map_err(|err| anyhow!("unable to do database migrations: {err}"))?;
    Ok(())
}

fn delete_old_unconfirmed_accounts(
    connection: &Connection,
    created_before: OffsetDateTime,
) -> anyhow::Result<()> {
    log::info!("Delete unconfirmed_accounts created before {created_before}");
    account::queries::delete_unconfirmed_accounts(&mut connection.0.lock(), created_before)
}

impl AccountRepo for Connection {
    fn find_account(&self, email: &EmailAddress) -> anyhow::Result<Option<AccountRecord>> {
        account::queries::fetch_account_from_db(&mut self.0.lock(), email)
    }

    fn save_account(&self, record: &AccountRecord) -> anyhow::Result<()> {
        account::queries::insert_or_update_account(&mut self.0.lock(), record.try_into()?)
    }

    fn delete_account(&self, email: &EmailAddress) -> anyhow::Result<()> {
        account::queries::delete_account_from_db(&mut self.0.lock(), email)
    }
}

impl AccountTokenRepo for Connection {
    fn replace_account_token(&self, account_token: AccountToken) -> anyhow::Result<EmailNonce> {
        account_token::queries::replace_account_token(&mut self.0.lock(), account_token)
    }

    fn consume_account_token(&self, email_nonce: &EmailNonce) -> anyhow::Result<AccountToken> {
        account_token::queries::consume_account_token(&mut self.0.lock(), email_nonce)
    }

    fn delete_expired_account_tokens(
        &self,
        expired_before: OffsetDateTime,
    ) -> anyhow::Result<usize> {
        account_token::queries::delete_expired_account_tokens(&mut self.0.lock(), expired_before)
    }

    fn get_account_token_by_email(&self, email: &EmailAddress) -> anyhow::Result<AccountToken> {
        account_token::queries::account_token_by_email(&mut self.0.lock(), email)
    }
}

impl ProjectRepo<boundary::JsonFormData> for Connection {
    fn find_project(&self, id: &ProjectId) -> Result<Option<Project>, anyhow::Error> {
        project::queries::find_project(&mut self.0.lock(), id)
    }

    fn all_projects_by_owner(&self, email: &EmailAddress) -> Result<Vec<Project>, anyhow::Error> {
        project::queries::all_projects_by_owner(&mut self.0.lock(), email)
    }

    fn save_project(&self, project: Project, owner: &EmailAddress) -> Result<(), anyhow::Error> {
        project::queries::save_project(&mut self.0.lock(), project, owner)
    }

    fn delete_project(&self, id: &ProjectId) -> Result<(), anyhow::Error> {
        project::queries::delete_project(&mut self.0.lock(), id)
    }
}
