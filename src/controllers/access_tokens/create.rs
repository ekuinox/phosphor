use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenCreateRequest {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenCreateResponse {
    success: bool,
    token: Option<String>
}

impl AccessTokenCreateResponse {
    pub fn new(success: bool, token: Option<String>) -> AccessTokenCreateResponse {
        AccessTokenCreateResponse { success: success, token: token }
    }
}

pub fn login(request: &AccessTokenCreateRequest, connection: &SqliteConnection) -> AccessTokenCreateResponse {
    match User::auth(request.username.clone(), request.password.clone(), &connection) {
        Some(user) => {
            match AccessToken::new(user.id.unwrap()).create(&connection) {
                Some(access_token) => AccessTokenCreateResponse::new(true, access_token.token),
                None => AccessTokenCreateResponse::new(false, None)
            }
        },
        None => AccessTokenCreateResponse::new(false, None)
    }
}
