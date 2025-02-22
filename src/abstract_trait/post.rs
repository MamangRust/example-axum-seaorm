use std::sync::Arc;

use crate::{
    domain::{
        ApiResponse, ApiResponsePagination, CreatePostRequest, ErrorResponse, FindAllPostRequest,
        PostRelationResponse, PostResponse, UpdatePostRequest,
    },
    entities::posts,
};
use async_trait::async_trait;
use sea_orm::DbErr;

pub type DynPostsRepository = Arc<dyn PostsRepositoryTrait + Send + Sync>;
pub type DynPostsService = Arc<dyn PostsServiceTrait + Send + Sync>;

#[async_trait]
pub trait PostsRepositoryTrait {
    async fn get_all_posts(
        &self,
        page: i32,
        page_size: i32,
        search: Option<String>,
    ) -> Result<(Vec<posts::Model>, i64), DbErr>;
    async fn get_post(&self, post_id: i32) -> Result<Option<posts::Model>, DbErr>;
    async fn get_post_relation(&self, post_id: i32) -> Result<Vec<PostRelationResponse>, DbErr>;
    async fn create_post(&self, input: &CreatePostRequest) -> Result<posts::Model, DbErr>;
    async fn update_post(&self, input: &UpdatePostRequest) -> Result<posts::Model, DbErr>;
    async fn delete_post(&self, post_id: i32) -> Result<(), DbErr>;
}

#[async_trait]
pub trait PostsServiceTrait {
    async fn get_all_posts(
        &self,
        req: FindAllPostRequest,
    ) -> Result<ApiResponsePagination<Vec<PostResponse>>, ErrorResponse>;
    async fn get_post(
        &self,
        post_id: i32,
    ) -> Result<Option<ApiResponse<PostResponse>>, ErrorResponse>;
    async fn get_post_relation(
        &self,
        post_id: i32,
    ) -> Result<ApiResponse<PostRelationResponse>, ErrorResponse>;
    async fn create_post(
        &self,
        input: &CreatePostRequest,
    ) -> Result<ApiResponse<PostResponse>, ErrorResponse>;
    async fn update_post(
        &self,
        input: &UpdatePostRequest,
    ) -> Result<ApiResponse<PostResponse>, ErrorResponse>;
    async fn delete_post(&self, post_id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}
