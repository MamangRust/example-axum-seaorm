use axum::{
    extract::{Extension, Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post, put}, Json
};
use serde_json::json;
use utoipa_axum::router::OpenApiRouter;
use std::sync::Arc;
use crate::{
    domain::{CategoryResponse, CreateCategoryRequest,ApiResponse,UpdateCategoryRequest}, middleware::jwt, state::AppState
};

#[utoipa::path(
    get,
    path = "/api/categories",
    responses(
        (status = 200, description = "List all category successfully", body = ApiResponse<Vec<CategoryResponse>>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
pub async fn get_categories(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.category_service.get_categories().await {
        Ok(categories) => Ok((
            StatusCode::OK,
            Json(json!(categories))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}

#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "List all category successfully", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
pub async fn get_category(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.category_service.get_category(id).await {
        Ok(Some(category)) => Ok((
            StatusCode::OK,
            Json(json!(category))
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Category not found"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}

#[utoipa::path(
    post,
    path = "/api/categories",
    responses(
        (status = 200, description = "Create category", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
pub async fn create_category(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCategoryRequest>,
  
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.category_service.create_category(&body).await {
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
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "Delete category", body = ApiResponse<CategoryResponse>)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
pub async fn update_category(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateCategoryRequest>,
   
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.id = Some(id);
    
    match data.di_container.category_service.update_category(&body).await {
        Ok(Some(category)) => Ok((
            StatusCode::OK,
            Json(json!(category))
        )),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "fail",
                "message": "Category not found"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}

#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    responses(
        (status = 200, description = "Delete category", body = Value)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "category"
)]
pub async fn delete_category(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>, // JWT middleware check
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.category_service.delete_category(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Category deleted successfully"
            }))
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!(e))
        ))
    }
}


pub fn category_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    let protected_routes = OpenApiRouter::new()
        .route("/api/categories/{id}", get(get_category))
        .route("/api/categories", post(create_category))
        .route("/api/categories/{id}", put(update_category))
        .route("/api/categories/{id}", delete(delete_category))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone());

    let public_routes = OpenApiRouter::new()
        .route("/categories", get(get_categories));

    OpenApiRouter::new()
        .merge(protected_routes)
        .merge(public_routes)
        .with_state(app_state.clone())
}
