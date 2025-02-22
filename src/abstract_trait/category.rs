use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DbErr;

use crate::{
    domain::{
        ApiResponse, ApiResponsePagination, CategoryResponse, CreateCategoryRequest, ErrorResponse,
        FindAllCategoryRequest, UpdateCategoryRequest,
    },
    entities::categories,
};

pub type DynCategoryRepository = Arc<dyn CategoryRepositoryTrait + Send + Sync>;
pub type DynCategoryService = Arc<dyn CategoryServiceTrait + Send + Sync>;

#[async_trait]
pub trait CategoryRepositoryTrait {
    async fn find_all(
        &self,
        page: i32,
        page_size: i32,
        search: Option<String>,
    ) -> Result<(Vec<categories::Model>, i64), DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<categories::Model>, DbErr>;
    async fn create(&self, input: &CreateCategoryRequest) -> Result<categories::Model, DbErr>;
    async fn update(&self, input: &UpdateCategoryRequest) -> Result<categories::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait CategoryServiceTrait {
    async fn get_categories(
        &self,
        req: FindAllCategoryRequest,
    ) -> Result<ApiResponsePagination<Vec<CategoryResponse>>, ErrorResponse>;
    async fn get_category(
        &self,
        id: i32,
    ) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse>;
    async fn create_category(
        &self,
        input: &CreateCategoryRequest,
    ) -> Result<ApiResponse<CategoryResponse>, ErrorResponse>;
    async fn update_category(
        &self,
        input: &UpdateCategoryRequest,
    ) -> Result<Option<ApiResponse<CategoryResponse>>, ErrorResponse>;
    async fn delete_category(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}
