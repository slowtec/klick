use std::{
    collections::HashMap,
    path::{self, PathBuf},
    sync::Arc,
    thread,
};

use anyhow::{anyhow, bail};
use axum::{
    extract::{Path, Query, State},
    http::{header, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Json, Response},
    routing::{delete, get, post, put},
    Router,
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use parking_lot::RwLock;
use rust_embed::RustEmbed;
use serde::Deserialize;
use time::{Duration, OffsetDateTime};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use url::Url;
use uuid::Uuid;

use klick_application_services as services;
use klick_boundary::{self as boundary, json_api};
use klick_db_sqlite::Connection;
use klick_domain::{Account, EmailAddress, EmailNonce, Password, ProjectId};
use klick_interfaces::{AccountRepo as _, ProjectRepo};
use klick_pdf_export::export_to_pdf;

mod adapters;
mod config;
mod notification_gateway;

use self::adapters::{ApiError, AuthError, LogoutError};

pub use self::config::Config;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../../frontend/dist/"]
struct Assets;

pub async fn run(config: &Config) -> anyhow::Result<()> {
    log::info!("Start KlicK server");
    let db = create_db_connection(config)?;
    let router = create_router(db, config)?;
    log::info!("Start listening on http://{}", config.address);
    let listener = TcpListener::bind(config.address).await?;
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}

const VALIDITY_PERIOD_OF_UNCONFIRMED_ACCOUNTS: Duration = Duration::days(2);

pub fn create_db_connection(config: &Config) -> anyhow::Result<Connection> {
    let db = Connection::establish(&config.db_connection)?;
    db.run_embedded_database_migrations()?;
    let created_before = OffsetDateTime::now_utc() - VALIDITY_PERIOD_OF_UNCONFIRMED_ACCOUNTS;
    db.delete_old_unconfirmed_accounts(created_before)?;
    Ok(db)
}

pub fn create_router(db: Connection, config: &Config) -> anyhow::Result<Router> {
    let base_url = config.base_url.clone();
    let notification_gw = notification_gateway::Gateway::new(config);
    let shared_state = AppState::new(db, base_url, notification_gw);

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::DELETE, Method::PUT, Method::POST])
        .allow_origin(Any);

    // TODO: rename users -> account
    let api = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/users", post(create_account))
        .route("/users", get(account_info))
        .route(
            "/users/resent-confirmation-email",
            post(resent_confirmation_email),
        )
        .route("/users/confirm-email-address", post(confirm_email_address))
        .route(
            "/users/reset-password-request",
            post(request_password_reset),
        )
        .route("/users/reset-password", post(reset_password))
        .route("/projects", get(get_all_projects))
        .route("/project", post(new_project))
        .route("/project/:id", put(update_project))
        .route("/project/:id", get(get_project))
        .route("/project/:id", delete(delete_project))
        .route("/project/:id/export", get(get_export))
        .route("/download/:download-id", get(get_download))
        .route("/download/:download-id/status", get(get_download_status))
        .route_layer(cors_layer)
        .with_state(shared_state);

    let router = Router::new().nest("/api", api).fallback(static_handler);
    Ok(router)
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html();
    }

    if let Some(content) = Assets::get(path) {
        let mime = mime_guess::from_path(path).first_or_octet_stream();

        ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
    } else {
        if path.contains('.') {
            return not_found();
        }
        index_html()
    }
}

fn index_html() -> Response {
    match Assets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found(),
    }
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}

#[derive(Clone)]
pub struct AppState {
    db: Connection,
    tokens: Arc<RwLock<HashMap<Uuid, Account>>>, // TODO: use stateless JWT
    downloads: Arc<RwLock<HashMap<Uuid, Download>>>, // TODO: use stateless JWT
    notification_gw: notification_gateway::Gateway,
    base_url: Url,
}

#[derive(Debug)]
struct Download {
    project_id: ProjectId,
    file_name: PathBuf,
    export_format: ExportFormat,
    status: DownloadStatus,
}

#[derive(Default, Debug)]
enum DownloadStatus {
    #[default]
    Pending,
    Completed(Vec<u8>),
    Failed(anyhow::Error),
}

#[derive(Debug, Clone, Copy)]
enum ExportFormat {
    Pdf,
    Json,
}

impl AppState {
    #[must_use]
    pub fn new(
        db: Connection,
        base_url: Url,
        notification_gw: notification_gateway::Gateway,
    ) -> Self {
        Self {
            db,
            base_url,
            tokens: Arc::default(),
            downloads: Arc::default(),
            notification_gw,
        }
    }
}

type Result<T> = std::result::Result<Json<T>, ApiError>;

async fn create_account(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<()> {
    let json_api::Credentials { email, password } = credentials;
    let email = email
        .parse::<EmailAddress>()
        .map_err(ApiError::CreateAccountEmail)?;
    let password = password
        .parse::<Password>()
        .map_err(ApiError::CreateAccountPassword)?;
    services::create_account(&state.db, &state.notification_gw, email, &password)?;
    Ok(Json(()))
}

async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<json_api::ApiToken> {
    let adapters::Credentials { email, password } = credentials.try_into()?;
    log::debug!("{email} tries to login");
    let account = services::login(&state.db, &email, &password)?;
    debug_assert_eq!(account.email_address, email);
    let token = Uuid::new_v4();
    state.tokens.write().insert(token, account);
    Ok(Json(json_api::ApiToken {
        token: token.to_string(),
    }))
}

async fn logout(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<()> {
    let token = auth
        .token()
        .parse::<Uuid>()
        .map_err(|_| LogoutError::NotLoggedIn)?;
    state.tokens.write().remove(&token);
    Ok(Json(()))
}

async fn account_info(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<json_api::UserInfo> {
    let account = account_from_token(&state, &auth)?;
    let email = account.email_address.into_string();
    let user_info = json_api::UserInfo { email };
    Ok(Json(user_info))
}

async fn confirm_email_address(
    State(state): State<AppState>,
    Json(data): Json<json_api::ConfirmEmailAddress>,
) -> Result<()> {
    let json_api::ConfirmEmailAddress { token } = data;
    let email_nonce = EmailNonce::decode_from_str(&token)?;
    services::confirm_email_address(state.db, &email_nonce)?;
    Ok(Json(()))
}

async fn resent_confirmation_email(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<()> {
    let adapters::Credentials { email, password } = credentials.try_into()?;
    log::debug!("{email} requests a new email to confirm");
    services::resend_confirmation_email(&state.db, &state.notification_gw, email, &password)
        .map_err(|err| {
            log::warn!("Unable to resent confirmation email: {err}");
            ApiError::InternalServerError
        })?;
    Ok(Json(()))
}

async fn request_password_reset(
    State(state): State<AppState>,
    Json(data): Json<json_api::RequestPasswordReset>,
) -> Result<()> {
    let json_api::RequestPasswordReset { email } = data;
    let email = email
        .parse::<EmailAddress>()
        .map_err(ApiError::LoginEmail)?;
    if let Err(err) = services::request_password_reset(&state.db, &state.notification_gw, email) {
        // We do not report any error to the user here,
        // but we create a log statement.
        log::warn!("Unable to request password reset: {err}");
    }
    Ok(Json(()))
}

async fn reset_password(
    State(state): State<AppState>,
    Json(data): Json<json_api::ResetPassword>,
) -> Result<()> {
    let json_api::ResetPassword {
        token,
        new_password,
    } = data;
    let password = new_password
        .parse::<Password>()
        .map(|pw| pw.to_hashed())
        .map_err(ApiError::LoginPassword)?;
    let email_nonce = EmailNonce::decode_from_str(&token)?;
    services::reset_password(state.db, email_nonce, password)?;
    Ok(Json(()))
}

async fn new_project(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<boundary::JsonFormData>,
) -> Result<boundary::ProjectId> {
    let account = account_from_token(&state, &auth)?;
    let id = services::create_new_project(&state.db, &account, data)?;
    let id = boundary::ProjectId::from(id);
    Ok(Json(id))
}

async fn update_project(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(updated): Json<boundary::SavedProject>,
) -> Result<()> {
    let account = account_from_token(&state, &auth)?;
    let id = ProjectId::from(updated.id);
    services::update_project(&state.db, &account, &id, updated.form_data)?;
    Ok(Json(()))
}

async fn get_project(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<boundary::SavedProject> {
    account_from_token(&state, &auth)?;
    let id = ProjectId::from_uuid(uuid);
    let project = services::read_project(&state.db, id)?;
    Ok(Json(project.into()))
}

#[derive(Deserialize)]
struct Export {
    format: Format,
    #[serde(rename = "file-name")]
    file_name: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Format {
    Json,
    Pdf,
}

async fn get_export(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Query(params): Query<Export>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<json_api::DownloadRequestResponse> {
    let account = account_from_token(&state, &auth)?;
    let project_id = ProjectId::from_uuid(uuid);
    log::debug!(
        "{} requested an {:?} export of project {project_id}",
        account.email_address,
        params.format
    );
    if state.db.find_project(&project_id)?.is_none() {
        let err = anyhow!("project {project_id} not found");
        log::warn!("{err}");
        return Err(ApiError::from(err));
    };
    let extension = match params.format {
        Format::Json => "json",
        Format::Pdf => "pdf",
    };
    let file_name = if let Some(file_name) = params.file_name {
        let Some(ext) = file_name.extension() else {
            let err = anyhow!("File name has no extension");
            return Err(ApiError::from(err));
        };
        if ext != extension {
            let err = anyhow!(
                "Invalid file extension ({}) expected: {extension}",
                ext.to_string_lossy()
            );
            return Err(ApiError::from(err));
        }
        file_name
    } else {
        path::Path::new("klimabilanz").with_extension(extension)
    };
    let export_format = match params.format {
        Format::Pdf => ExportFormat::Pdf,
        Format::Json => ExportFormat::Json,
    };

    let download = Download {
        project_id,
        file_name,
        export_format,
        status: DownloadStatus::default(),
    };

    let download_id = Uuid::new_v4();
    state.downloads.write().insert(download_id, download);

    start_background_download_task(download_id, state.clone());
    let download_id = json_api::DownloadId(download_id);
    Ok(Json(json_api::DownloadRequestResponse { download_id }))
}

async fn get_download(
    State(state): State<AppState>,
    Path(download_id): Path<Uuid>,
) -> std::result::Result<Response, ApiError> {
    log::debug!("Download {download_id}");
    let Some(download) = state.downloads.write().remove(&download_id) else {
        let err = anyhow!("download {download_id} not found");
        log::warn!("{err}");
        return Err(ApiError::from(err));
    };
    match download.status {
        DownloadStatus::Pending => {
            let err = anyhow!("Download {download_id} was not ready.");
            Err(ApiError::from(err))
        }
        DownloadStatus::Failed(err) => {
            let err = anyhow!("Download {download_id} failed: {err}");
            Err(ApiError::from(err))
        }
        DownloadStatus::Completed(bytes) => {
            let headers = match download.export_format {
                ExportFormat::Pdf => [
                    (header::CONTENT_TYPE, "application/pdf"),
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename={}", download.file_name.display()),
                    ),
                ],
                ExportFormat::Json => [
                    (header::CONTENT_TYPE, "application/json; charset=utf-8"),
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename={}", download.file_name.display()),
                    ),
                ],
            };
            Ok((headers, bytes).into_response())
        }
    }
}

async fn get_download_status(
    State(state): State<AppState>,
    Path(download_id): Path<Uuid>,
) -> Result<json_api::DownloadStatus> {
    let read_lock = state.downloads.read();
    let Some(download) = read_lock.get(&download_id) else {
        let err = anyhow!("Download {download_id} not found");
        return Err(ApiError::from(err));
    };
    let status = match &download.status {
        DownloadStatus::Pending => json_api::DownloadStatus::Pending,
        DownloadStatus::Failed(err) => json_api::DownloadStatus::Failed(err.to_string()),
        DownloadStatus::Completed(_) => {
            let download_url = state
                .base_url
                .join(&format!("/api/download/{download_id}"))
                .unwrap()
                .to_string();
            json_api::DownloadStatus::Completed(download_url)
        }
    };
    Ok(Json(status))
}

async fn get_all_projects(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Vec<boundary::SavedProject>> {
    let account = account_from_token(&state, &auth)?;
    let projects = services::read_all_projects(&state.db, &account)?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(projects))
}

async fn delete_project(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<()> {
    account_from_token(&state, &auth)?;
    let id = ProjectId::from_uuid(uuid);
    services::delete_project(&state.db, id)?;
    Ok(Json(()))
}

fn account_from_token(
    state: &AppState,
    auth: &Authorization<Bearer>,
) -> std::result::Result<Account, ApiError> {
    let token = auth
        .token()
        .parse::<Uuid>()
        .map_err(|_| AuthError::NotAuthorized)?;
    let account = state
        .tokens
        .read()
        .get(&token)
        .cloned()
        .ok_or(AuthError::NotAuthorized)?;

    // check if account still exits
    let Some(record) = state
        .db
        .find_account(&account.email_address)
        .map_err(|err| {
            log::warn!("Unable to find account: {err}");
            ApiError::InternalServerError
        })?
    else {
        return Err(AuthError::NotAuthorized.into());
    };
    if !record.account.email_confirmed {
        return Err(AuthError::EmailNotConfirmed.into());
    }
    Ok(account)
}

fn start_background_download_task(download_id: Uuid, state: AppState) {
    thread::spawn(move || {
        let (project_id, export_format): (ProjectId, ExportFormat) = {
            let read_lock = state.downloads.read();
            let Some(download) = read_lock.get(&download_id) else {
                log::warn!("Download {download_id} not found: exit download task");
                return;
            };
            (download.project_id, download.export_format)
        };

        let result = download_task(project_id, export_format, &state.db);

        let mut write_lock = state.downloads.write();
        let Some(download) = write_lock.get_mut(&download_id) else {
            log::warn!("Download {download_id} not found: exit download task");
            return;
        };
        match result {
            Ok(bytes) => {
                download.status = DownloadStatus::Completed(bytes);
            }
            Err(err) => {
                download.status = DownloadStatus::Failed(err);
            }
        }
    });
}

fn download_task(
    project_id: ProjectId,
    format: ExportFormat,
    db: &Connection,
) -> anyhow::Result<Vec<u8>> {
    let Some(project) = db.find_project(&project_id)? else {
        bail!("Project {project_id} not found");
    };
    let project = boundary::Project::from(project);

    match format {
        ExportFormat::Pdf => {
            let form_data: HashMap<_, _> = project.into_form_data().try_into()?;
            let values = form_data
                .into_iter()
                .map(|(id, value)| (id.into(), value))
                .collect();
            export_to_pdf(&values)
        }
        ExportFormat::Json => {
            let json_string = boundary::export_to_string_pretty(&project);
            Ok(json_string.into_bytes())
        }
    }
}
