use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::State,
    http::{header, Method, StatusCode, Uri},
    response::{Html, IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization};
use parking_lot::RwLock;
use rust_embed::RustEmbed;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use klick_application::usecases;
use klick_boundary::json_api;
use klick_db_sqlite::Connection;
use klick_domain::{Account, EmailAddress, EmailNonce, Password};

mod adapters;
mod config;
mod notification_gateway;

use self::adapters::*;

pub use self::config::Config;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../../frontend/dist/"]
struct Assets;

pub async fn run(config: Config) -> anyhow::Result<()> {
    log::info!("Start KlicK server");

    let db = Connection::establish(&config.db_connection)?;
    db.run_embedded_database_migrations()?;

    let notification_gw = notification_gateway::Gateway::new(&config);

    let shared_state = AppState::new(db, notification_gw);

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let api = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/users", post(create_account))
        .route("/users", get(account_info))
        .route("/users/confirm-email-address", post(confirm_email_address))
        .route(
            "/users/reset-password-request",
            post(request_password_reset),
        )
        .route("/users/reset-password", post(reset_password))
        .route_layer(cors_layer)
        .with_state(shared_state);

    let app = Router::new().nest("/api", api).fallback(static_handler);

    log::info!("Start listening on http://{}", config.address);
    let listener = TcpListener::bind(config.address).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
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
    tokens: Arc<RwLock<HashMap<Uuid, Account>>>,
    notification_gw: notification_gateway::Gateway,
}

impl AppState {
    pub fn new(db: Connection, notification_gw: notification_gateway::Gateway) -> Self {
        Self {
            db,
            tokens: Default::default(),
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
    usecases::create_account(state.db, state.notification_gw, email, &password)?;
    Ok(Json(()))
}

async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<json_api::ApiToken> {
    let json_api::Credentials { email, password } = credentials;
    log::debug!("{email} tries to login");
    let email = email
        .parse::<EmailAddress>()
        .map_err(ApiError::LoginEmail)?;
    let password = password
        .parse::<Password>()
        .map_err(ApiError::LoginPassword)?;
    let account = usecases::login(state.db, &email, &password)?;
    debug_assert_eq!(account.email, email);
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
    let token = auth
        .token()
        .parse::<Uuid>()
        .map_err(|_| AuthError::NotAuthorized)?;
    let email = state
        .tokens
        .read()
        .get(&token)
        .map(|account| account.email.clone())
        .ok_or(AuthError::NotAuthorized)?;
    let email = email.into_string();
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

async fn request_password_reset(
    State(state): State<AppState>,
    Json(data): Json<json_api::RequestPasswordReset>,
) -> Result<()> {
    let json_api::RequestPasswordReset { email } = data;
    let email = email
        .parse::<EmailAddress>()
        .map_err(ApiError::LoginEmail)?;
    if let Err(err) = usecases::request_password_reset(state.db, state.notification_gw, email) {
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
