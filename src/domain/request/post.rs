use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub body: String,
    pub img: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub post_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub img: String,
    pub category_id: i32,
    pub user_id: i32,
    pub user_name: String,
}