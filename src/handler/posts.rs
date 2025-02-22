use crate::{
    domain::{
        ApiResponse, ApiResponsePagination, CreatePostRequest, FindAllPostRequest,
        PostRelationResponse, PostResponse, UpdatePostRequest,
    },
    middleware::jwt,
    state::AppState,
};
use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json,
};
use serde_json::json;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/posts",
    params(FindAllPostRequest),
    responses(
        (status = 200, description = "List all posts successfully", body = ApiResponsePagination<Vec<PostResponse>>)
    ),
    security(("bearer_auth" = [])),
    tag = "post"
)]
pub async fn get_posts(
    State(data): State<Arc<AppState>>,
    Query(params): Query<FindAllPostRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.get_all_posts(params).await {
        Ok(posts) => Ok((StatusCode::OK, Json(json!(posts)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}",
    params(
            ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Get post by ID", body = ApiResponse<PostResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post(
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.get_post(post_id).await {
        Ok(Some(post)) => Ok((StatusCode::OK, Json(json!(post)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Post not found"
            })),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/posts/{id}/relation",
    params(
            ("id" = i32, Path, description = "Post ID")
    ),
    responses(
        (status = 200, description = "Get related posts", body = ApiResponse<Vec<PostRelationResponse>>),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post_relation(
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .post_service
        .get_post_relation(post_id)
        .await
    {
        Ok(posts) => Ok((StatusCode::OK, Json(json!(posts)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
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
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut post_data: Option<CreatePostRequest> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Failed to process form data"})),
        )
    })? {
        if field.name() == Some("post_data") {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Failed to read post data"})),
                )
            })?;
            post_data = Some(serde_json::from_slice(&data).map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid post data format"})),
                )
            })?);
            break;
        }
    }

    let mut post_data = post_data.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing post data"})),
        )
    })?;

    let upload_result = data
        .di_container
        .file_service
        .upload_image("posts", multipart)
        .await;

    let uploaded_file_name = match upload_result {
        Ok(response) => response.file_name.clone(),
        Err((status, response)) => {
            return Err((
                status,
                Json(json!({
                    "error": response.message
                })),
            ))
        }
    };

    post_data.img = uploaded_file_name;

    match data.di_container.post_service.create_post(&post_data).await {
        Ok(post) => Ok((StatusCode::CREATED, Json(json!(post)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )),
    }
}

#[utoipa::path(
    put,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID")
    ),
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
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut post_data: Option<UpdatePostRequest> = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Failed to process form data"})),
        )
    })? {
        if field.name() == Some("post_data") {
            let data = field.bytes().await.map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Failed to read post data"})),
                )
            })?;
            let mut post_req: UpdatePostRequest = serde_json::from_slice(&data).map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid post data format"})),
                )
            })?;
            post_req.post_id = Some(post_id);
            post_data = Some(post_req);
            break;
        }
    }

    let mut post_data = post_data.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing post data"})),
        )
    })?;

    let upload_result = data
        .di_container
        .file_service
        .upload_image("posts", multipart)
        .await;

    let uploaded_file_name = match upload_result {
        Ok(response) => response.file_name.clone(),
        Err((status, response)) => {
            return Err((
                status,
                Json(json!({
                    "error": response.message
                })),
            ))
        }
    };

    post_data.img = uploaded_file_name;

    match data.di_container.post_service.update_post(&post_data).await {
        Ok(post) => Ok((StatusCode::OK, Json(json!(post)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/api/posts/{id}",
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
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
pub async fn delete_post(
    State(data): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.post_service.delete_post(post_id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Post deleted successfully"
            })),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub fn post_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    let protected_routes = OpenApiRouter::new()
        .route("/api/posts", post(create_post))
        .route("/api/posts/{id}", get(get_post))
        .route("/api/posts/{id}", put(update_post))
        .route("/api/posts/{id}", delete(delete_post))
        .route("/api/posts/{id}/relation", get(get_post_relation))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone());

    let public_routes = OpenApiRouter::new().route("/posts", get(get_posts));

    OpenApiRouter::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(app_state)
}
