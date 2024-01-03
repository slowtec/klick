use std::{net::SocketAddr, sync::Arc};

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
use thiserror::Error;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use klick_boundary::json_api;

mod adapters;
mod application;

use self::application::*;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../../frontend/dist/"]
struct Assets;

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    log::info!("Start KlicK server");

    let shared_state = Arc::new(RwLock::new(AppState::default()));

    let cors_layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let api = Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/users", post(create_user))
        .route("/users", get(get_user_info))
        .route_layer(cors_layer)
        .with_state(shared_state);

    let app = Router::new().nest("/api", api).fallback(static_handler);

    log::info!("Start listening on http://{addr}");
    let listener = TcpListener::bind(addr).await?;
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

type Result<T> = std::result::Result<Json<T>, Error>;

/// API error
#[derive(Error, Debug)]
#[non_exhaustive]
enum Error {
    #[error(transparent)]
    CreateUser(#[from] CreateUserError),
    #[error(transparent)]
    Login(#[from] LoginError),
    #[error(transparent)]
    Logout(#[from] LogoutError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Credentials(#[from] adapters::CredentialParsingError),
}

async fn create_user(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<()> {
    let credentials = Credentials::try_from(credentials)?;
    state.write().create_user(credentials)?;
    Ok(Json(()))
}

async fn login(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<json_api::ApiToken> {
    let json_api::Credentials { email, password } = credentials;
    log::debug!("{email} tries to login");
    let email = email.parse::<EmailAddress>().map_err(|_|
          // Here we don't want to leak detailed info.
          LoginError::InvalidEmailOrPassword)?;
    let token = state
        .write()
        .login(email, &password)
        .map(|s| s.to_string())?;
    Ok(Json(json_api::ApiToken { token }))
}

async fn logout(
    State(state): State<Arc<RwLock<AppState>>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<()> {
    state.write().logout(auth.token())?;
    Ok(Json(()))
}

async fn get_user_info(
    State(state): State<Arc<RwLock<AppState>>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<json_api::UserInfo> {
    let user = state.read().authorize_user(auth.token())?;
    let CurrentUser { email, .. } = user;
    let email = email.into_string();
    let user_info = json_api::UserInfo { email };
    Ok(Json(user_info))
}
