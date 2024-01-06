use std::{collections::HashMap, net::SocketAddr, sync::Arc};

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
use klick_domain::{EmailAddress, Password, User};

mod adapters;
use self::adapters::*;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../../frontend/dist/"]
struct Assets;

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    log::info!("Start KlicK server");

    let db = Connection::establish("db.sqlite")?;
    db.run_embedded_database_migrations()?;

    let shared_state = AppState::new(db);

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

#[derive(Clone)]
pub struct AppState {
    db: Connection,
    tokens: Arc<RwLock<HashMap<Uuid, User>>>,
}

impl AppState {
    pub fn new(db: Connection) -> Self {
        Self {
            db,
            tokens: Default::default(),
        }
    }
}

type Result<T> = std::result::Result<Json<T>, ApiError>;

async fn create_user(
    State(state): State<AppState>,
    Json(credentials): Json<json_api::Credentials>,
) -> Result<()> {
    let json_api::Credentials { email, password } = credentials;
    let email = email
        .parse::<EmailAddress>()
        .map_err(ApiError::CreateUserEmail)?;
    let password = password
        .parse::<Password>()
        .map_err(ApiError::CreateUserPassword)?;
    usecases::create_user(state.db, email, &password)?;
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
    let user = usecases::login(state.db, &email, &password)?;
    debug_assert_eq!(user.email, email);
    let token = Uuid::new_v4();
    state.tokens.write().insert(token, user);
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

async fn get_user_info(
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
        .map(|user| user.email.clone())
        .ok_or(AuthError::NotAuthorized)?;
    let email = email.into_string();
    let user_info = json_api::UserInfo { email };
    Ok(Json(user_info))
}
