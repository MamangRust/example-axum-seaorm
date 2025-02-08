use axum::{
    extract::{Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post, put}, Json
};
use serde_json::json;
use utoipa_axum::router::OpenApiRouter;
use std::sync::Arc;
use crate::{
    middleware::jwt, domain::{ApiResponse, CreatePostRequest, PostRelationResponse, PostResponse, UpdatePostRequest}, state::AppState
};

#[utoipa::path(
    get,
    path = "/api/posts",
    responses(
        (status = 200, description = "Get list of posts", body = ApiResponse<Vec<PostResponse>>)
    ),
    tag = "posts"
)]
pub async fn get_posts(State(data): State<Arc<AppState>>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Get post by ID", body = ApiResponse<PostResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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


#[utoipa::path(
    get,
    path = "/api/posts/{id}/relation",
    responses(
        (status = 200, description = "Get related posts", body = ApiResponse<Vec<PostRelationResponse>>),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post_relation(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

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


#[utoipa::path(
    post,
    path = "/api/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = ApiResponse<PostResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
pub async fn create_post(
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

#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated successfully", body = ApiResponse<PostResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 404, description = "Post not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
pub async fn update_post(
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

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 404, description = "Post not found"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "posts"
)]
pub async fn delete_post(State(data): State<Arc<AppState>>, Path(post_id): Path<i32>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

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


pub fn post_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    let protected_routes = OpenApiRouter::new()
        .route("/api/posts", post(create_post))
        .route("/api/posts/{id}", get(get_post))
        .route("/api/posts/{id}", put(update_post))
        .route("/api/posts/{id}", delete(delete_post))
        .route("/api/posts/{id}/relation", get(get_post_relation))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth)) // Middleware autentikasi
        .with_state(app_state.clone());

    let public_routes = OpenApiRouter::new()
        .route("/posts", get(get_posts));

        OpenApiRouter::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(app_state)
}
