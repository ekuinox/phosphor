use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, user::BasicCredentials};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
    token: Option<String>
}

impl Response {
    pub fn new(success: bool, token: Option<String>) -> Response {
        Response { success: success, token: token }
    }
}

pub fn create(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(BasicCredentials::new(request.username.clone(), request.password.clone()), &connection) {
        Some(user) => {
            match AccessToken::new(user.id.unwrap()).create(&connection) {
                Some(access_token) => Response::new(true, access_token.token),
                None => Response::new(false, None)
            }
        },
        None => Response::new(false, None)
    }
}
