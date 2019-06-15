use crate::models::user::User;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool
}

impl Response {
    pub fn new(success: bool) -> Response {
        Response { success: success }
    }
}

pub fn login(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(request.username.clone(), request.password.clone(), &connection) {
        Some(_) => Response::new(true),
        None => Response::new(false)
    }
}
