use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct UpdateCategoryRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
}