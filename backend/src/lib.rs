use std::net::SocketAddr;

use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
    Router,
};
use rust_embed::RustEmbed;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "../frontend/dist/"]
struct Assets;

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    log::info!("Start KlicK server");
    let app = Router::new().fallback(static_handler);

    log::info!("Start listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
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
