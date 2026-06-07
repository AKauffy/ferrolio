mod handlers;
mod post;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "portfolio_backend=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Eagerly load posts at startup so any parse errors surface immediately
    let post_count = post::store::load_posts().len();
    tracing::info!("Loaded {} posts from disk", post_count);

    let cors = CorsLayer::new()
        .allow_origin(
            std::env::var("ALLOWED_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".into())
                .parse::<axum::http::HeaderValue>()
                .expect("Invalid ALLOWED_ORIGIN"),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/posts", get(handlers::posts::list_posts))
        .route("/posts/:slug", get(handlers::posts::get_post))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
