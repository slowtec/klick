use std::sync::Arc;

use anyhow::anyhow;
use diesel::{sqlite::SqliteConnection, Connection as _};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use parking_lot::Mutex;
use thiserror::Error;

use klick_application::{UserRecord, UserRepo};
use klick_domain::EmailAddress;

use crate::user;

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
        log::info!("Running embedded user database migrations");
        run_embedded_database_migrations(self, MIGRATIONS)
    }
}

impl UserRepo for Connection {
    fn find_user(&self, email: &EmailAddress) -> anyhow::Result<Option<UserRecord>> {
        user::queries::fetch_user_from_db(&mut self.0.lock(), email)
    }

    fn save_user(&self, record: &UserRecord) -> anyhow::Result<()> {
        user::queries::insert_or_update_user(&mut self.0.lock(), record.try_into()?)
    }

    fn delete_user(&self, email: &EmailAddress) -> anyhow::Result<()> {
        user::queries::delete_user_from_db(&mut self.0.lock(), email)
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
    connection
        .0
        .lock()
        .run_pending_migrations(migrations)
        .map_err(|err| anyhow!("unable to do database migrations: {err}"))?;
    Ok(())
}
