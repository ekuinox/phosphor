use crate::models::user::User;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginRequest {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponse {
    success: bool
}

impl UserLoginResponse {
    pub fn new(success: bool) -> UserLoginResponse {
        UserLoginResponse { success: success }
    }
}

pub fn login(request: &UserLoginRequest, connection: &SqliteConnection) -> UserLoginResponse {
    match User::auth(request.username.clone(), request.password.clone(), &connection) {
        Some(_) => UserLoginResponse::new(true),
        None => UserLoginResponse::new(false)
    }
}
