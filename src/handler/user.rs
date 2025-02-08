use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    middleware,
    routing::{delete, get, post, put},
};
use serde_json::json;
use utoipa_axum::router::OpenApiRouter;
use std::sync::Arc;
use crate::{
    domain::{ApiResponse, CreateUserRequest, UpdateUserRequest, UserResponse},
    middleware::jwt,
    state::AppState,
};

#[utoipa::path(
    post,
    path = "/api/user",
    responses(
        (status = 200, description = "Create user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn create_user(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.user_service.create_user(&body).await {
        Ok(response) => Ok((StatusCode::CREATED, Json(json!(response)))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e)),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/api/user/{email}",
    responses(
        (status = 200, description = "Find Email user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn find_user_by_email(
    State(data): State<Arc<AppState>>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.user_service.find_user_by_email(&email).await {
        Ok(Some(response)) => Ok((StatusCode::OK, Json(json!(response)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "User not found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e)),
        )),
    }
}

#[utoipa::path(
    put,
    path = "/api/user/{id}",
    responses(
        (status = 200, description = "Update user", body = ApiResponse<UserResponse>),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn update_user(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.id = Some(id);

    match data.di_container.user_service.update_user(&body).await {
        Ok(Some(response)) => Ok((StatusCode::OK, Json(json!(response)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "User not found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e)),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/api/user/{email}",
    responses(
        (status = 200, description = "User category", body = Value),
        (status = 400, description = "Invalid request body"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "users"
)]
pub async fn delete_user(
    State(data): State<Arc<AppState>>,
    Path(email): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.user_service.delete_user(&email).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "User deleted successfully"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e)),
        )),
    }
}


pub fn user_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    let protected_routes = OpenApiRouter::new()
        .route("/api/user", post(create_user))
        .route("/api/user/email/{email}", get(find_user_by_email))
        .route("/api/user/id/{id}", put(update_user))
        .route("/api/user/{email}", delete(delete_user))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone());

        OpenApiRouter::new().merge(protected_routes).with_state(app_state)
}