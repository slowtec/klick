use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
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

use klick_application::{usecases, AccountRepo as _, ProjectRepo};
use klick_boundary::{self as boundary, json_api};
use klick_db_sqlite::Connection;
use klick_domain::{
    authentication::{Account, EmailAddress, EmailNonce, Password},
    ProjectId,
};
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
        .route("/download/:id/:filename", get(get_download))
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

enum Download {
    PdfReport { project_id: ProjectId },
    JsonProject { project_id: ProjectId },
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
            tokens: Default::default(),
            downloads: Default::default(),
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
    usecases::create_account(&state.db, &state.notification_gw, email, &password)?;
    Ok(Json(()))
}

async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<json_api::ApiToken> {
    let adapters::Credentials { email, password } = credentials.try_into()?;
    log::debug!("{email} tries to login");
    let account = usecases::login(&state.db, &email, &password)?;
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
    let account = account_from_token(&state, auth)?;
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
    usecases::confirm_email_address(state.db, &email_nonce)?;
    Ok(Json(()))
}

async fn resent_confirmation_email(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<()> {
    let adapters::Credentials { email, password } = credentials.try_into()?;
    log::debug!("{email} requests a new email to confirm");
    usecases::resend_confirmation_email(&state.db, &state.notification_gw, email, &password)
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
    if let Err(err) = usecases::request_password_reset(&state.db, &state.notification_gw, email) {
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
    usecases::reset_password(state.db, email_nonce, password)?;
    Ok(Json(()))
}

async fn new_project(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<boundary::JsonFormData>,
) -> Result<boundary::ProjectId> {
    let account = account_from_token(&state, auth)?;
    let id = usecases::create_new_project(&state.db, &account, data)?;
    let id = boundary::ProjectId::from(id);
    Ok(Json(id))
}

async fn update_project(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(updated): Json<boundary::SavedProject>,
) -> Result<()> {
    let account = account_from_token(&state, auth)?;
    let id = ProjectId::from_uuid(updated.id.0);
    usecases::update_project(&state.db, &account, &id, updated.data)?;
    Ok(Json(()))
}

async fn get_project(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<boundary::SavedProject> {
    account_from_token(&state, auth)?;
    let id = ProjectId::from_uuid(uuid);
    let project = usecases::read_project(&state.db, id)?;
    Ok(Json(project.into()))
}

#[derive(Deserialize)]
struct Export {
    format: Format,
}

#[derive(Deserialize)]
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
    account_from_token(&state, auth)?;
    let id = ProjectId::from_uuid(uuid);
    log::debug!("Export project {id:?}");
    if state.db.find_project(&id)?.is_none() {
        return Err(ApiError::from(anyhow!("project not found")));
    };
    let file_name = match params.format {
        Format::Json => "klimabilanz.json",
        Format::Pdf => "klimabilanz.pdf",
    };
    let download = match params.format {
        Format::Pdf => Download::PdfReport { project_id: id },
        Format::Json => Download::JsonProject { project_id: id },
    };
    let download_id = Uuid::new_v4();
    state.downloads.write().insert(download_id, download);
    let download_url = state
        .base_url
        .join(&format!("/api/download/{download_id}/{file_name}"))
        .unwrap()
        .to_string();

    Ok(Json(json_api::DownloadRequestResponse { download_url }))
}

async fn get_download(
    State(state): State<AppState>,
    Path((download_id, filename)): Path<(Uuid, std::path::PathBuf)>,
) -> std::result::Result<Response, ApiError> {
    let Some(download) = state.downloads.write().remove(&download_id) else {
        return Err(ApiError::from(anyhow!("download not found")));
    };
    match download {
        Download::PdfReport { project_id } => {
            let Some(project) = state.db.find_project(&project_id)? else {
                return Err(ApiError::from(anyhow!("project not found")));
            };
            let project = boundary::Project::from(project);
            let project_data = match project {
                boundary::Project::Saved(p) => p.data,
                boundary::Project::Unsaved(d) => d,
            };
            // FIXME: check if pdf was alread made
            let bytes = export_to_pdf(project_data.into())?;
            let headers = [
                (header::CONTENT_TYPE, "application/pdf"),
                (
                    header::CONTENT_DISPOSITION,
                    &format!("attachment; filename={}", filename.display()),
                ),
            ];
            Ok((headers, bytes).into_response())
        }
        Download::JsonProject { project_id } => {
            let Some(project) = state.db.find_project(&project_id)? else {
                return Err(ApiError::from(anyhow!("project not found")));
            };
            let project = boundary::Project::from(project);
            let data = boundary::Data { project };
            let json_string = boundary::export_to_string_pretty(&data);
            let headers = [
                (header::CONTENT_TYPE, "application/json; charset=utf-8"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=report.json",
                ),
            ];
            Ok((headers, json_string).into_response())
        }
    }
}

async fn get_all_projects(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<Vec<boundary::SavedProject>> {
    let account = account_from_token(&state, auth)?;
    let projects = usecases::read_all_projects(&state.db, &account)?
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
    account_from_token(&state, auth)?;
    let id = ProjectId::from_uuid(uuid);
    usecases::delete_project(&state.db, id)?;
    Ok(Json(()))
}

fn account_from_token(
    state: &AppState,
    auth: Authorization<Bearer>,
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
