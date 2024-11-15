use axum::{
    extract::{Extension, Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post, put}, Json, Router
};
use serde_json::json;
use std::sync::Arc;
use crate::{
    middleware::jwt, domain::{CreateCategoryRequest, UpdateCategoryRequest}, state::AppState
};

// Get all categories handler
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

// Get single category handler
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

// Create category handler
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

// Update category handler
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

// Delete category handler
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


pub fn category_routes(app_state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
        .route("/api/categories/:id", get(get_category))
        .route("/api/categories", post(create_category))
        .route("/api/categories/:id", put(update_category))
        .route("/api/categories/:id", delete(delete_category))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth)).with_state(app_state.clone());

    let public_routes = Router::new()
        .route("/categories", get(get_categories));

   
     Router::new()
        .merge(protected_routes)
        .merge(public_routes).with_state(app_state.clone())
}

// // In main.rs or wherever you set up your app:
// pub fn app(app_state: Arc<AppState>) -> Router {
//     Router::new()
//         .merge(category_routes(app_state.clone()))
       
// }