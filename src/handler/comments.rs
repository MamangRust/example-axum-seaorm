use axum::{
    extract::{State, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use crate::{
    middleware::jwt,
    domain::{CreateCommentRequest, UpdateCommentRequest},
    state::AppState,
};


async fn get_comments(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.comment_service.get_comments().await {
        Ok(comments) => Ok((StatusCode::OK, Json(json!(comments)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to fetch comments",
                "error": format!("{:?}", e)
            })),
        )),
    }
}

// Handler untuk mendapatkan komentar berdasarkan ID
async fn get_comment(
    State(data): State<Arc<AppState>>,
    Path(comment_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.comment_service.get_comment(comment_id).await {
        Ok(Some(comment)) => Ok((StatusCode::OK, Json(json!(comment)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Comment not found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to fetch comment",
                "error": format!("{:?}", e)
            })),
        )),
    }
}

// Handler untuk membuat komentar baru
async fn create_comment(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.comment_service.create_comment(&body).await {
        Ok(comment) => Ok((StatusCode::CREATED, Json(json!(comment)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to create comment",
                "error": format!("{:?}", e)
            })),
        )),
    }
}

// Handler untuk memperbarui komentar
async fn update_comment(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateCommentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.comment_service.update_comment(&body).await {
        Ok(Some(comment)) => Ok((StatusCode::OK, Json(json!(comment)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Comment not found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to update comment",
                "error": format!("{:?}", e)
            })),
        )),
    }
}

// Handler untuk menghapus komentar
async fn delete_comment(
    State(data): State<Arc<AppState>>,
    Path(comment_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.comment_service.delete_comment(comment_id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Comment deleted successfully"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "status": "error",
                "message": "Failed to delete comment",
                "error": format!("{:?}", e)
            })),
        )),
    }
}

pub fn comment_routes(app_state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/api/comments", get(get_comments))
        .route("/api/comments/:id", get(get_comment))
        .route("/api/comments", post(create_comment))
        .route("/api/comments/:id", put(update_comment))
        .route("/api/comments/:id", delete(delete_comment))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone());

    Router::new().merge(protected_routes).with_state(app_state)
}