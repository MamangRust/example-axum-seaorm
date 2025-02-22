use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::Multipart, http::StatusCode, Json};

use crate::domain::{DeleteResponse, UploadResponse};

pub type DynFileService = Arc<dyn FileServiceTrait + Send + Sync>;

#[async_trait]
pub trait FileServiceTrait {
    async fn upload_image(
        &self,
        upload_dir: &str,
        mut multipart: Multipart,
    ) -> Result<Json<UploadResponse>, (StatusCode, Json<UploadResponse>)>;
    async fn delete_image(
        &self,
        upload_dir: &str,
        file_name: &str,
    ) -> Result<Json<DeleteResponse>, (StatusCode, Json<DeleteResponse>)>;
}
