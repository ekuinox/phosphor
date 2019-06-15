use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::user::{User, Authenticate, BasicCredentials};

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
    match User::auth(BasicCredentials::new(request.username.clone(), request.password.clone()), &connection) {
        Some(_) => Response::new(true),
        None => Response::new(false)
    }
}
