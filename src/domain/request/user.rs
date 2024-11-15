use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: Option<i32>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
   
    pub email: Option<String>, // Option since it might not be changed
    pub password: Option<String>,
}

