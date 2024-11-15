use axum::{
    extract::{Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post, put}, Json, Router
};
use serde_json::json;
use std::sync::Arc;
use crate::{
    middleware::jwt, domain::{CreatePostRequest, UpdatePostRequest}, state::AppState
};

async fn get_posts(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.get_all_posts().await {
        Ok(posts) => Ok((
            StatusCode::OK,
            Json(json!(posts))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


async fn get_post(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.get_post(post_id).await {
        Ok(Some(post)) => Ok((
            StatusCode::OK,
            Json(json!(post))
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Post not found"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


async fn get_post_relation(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    match data.di_container.post_service.get_post_relation(post_id).await {
        Ok(posts) => Ok((
            StatusCode::OK,
            Json(json!(posts))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


async fn create_post(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.create_post(&body).await {
        Ok(category) => Ok((
            StatusCode::CREATED,
            Json(json!(category))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


async fn update_post(
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Json(mut body): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.post_id = Some(post_id);


    match data.di_container.post_service.update_post(&body).await {
        Ok(post) => Ok((
            StatusCode::OK,
            Json(json!(post))
        )),
        
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }

}


async fn delete_post(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    match data.di_container.post_service.delete_post(post_id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Post deleted successfully"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
   
}


pub fn post_routes(app_state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/api/posts", post(create_post))
        .route("/api/posts/:id", get(get_post))
        .route("/api/posts/:id", put(update_post))
        .route("/api/posts/:id", delete(delete_post))
        .route("/api/posts/:id/relation", get(get_post_relation))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth)) // Middleware autentikasi
        .with_state(app_state.clone());

    let public_routes = Router::new()
        .route("/posts", get(get_posts));

    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(app_state)
}
