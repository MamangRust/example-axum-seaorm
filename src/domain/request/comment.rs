use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCommentRequest {
    pub id_post_comment: i32,
    pub user_name_comment: String,
    pub comment: String,
}