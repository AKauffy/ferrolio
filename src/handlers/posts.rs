use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;

use crate::post::store;

pub async fn list_posts() -> Json<serde_json::Value> {
    let summaries = store::all_summaries();
    Json(json!({ "posts": summaries }))
}

pub async fn get_post(Path(slug): Path<String>) -> Response {
    match store::find_by_slug(&slug) {
        Some(post) => Json(json!({
            "slug": post.slug,
            "meta": post.meta,
            "content_html": post.content_html,
        }))
        .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Post not found" })),
        )
            .into_response(),
    }
}
