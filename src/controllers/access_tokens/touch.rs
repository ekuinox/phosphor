use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::*, user::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
    user: Option<PrivateUserData>
}

pub fn touch(request: &Request, connection: &SqliteConnection) -> Response {
    match AccessToken::from_string(&request.token, &connection) {
        Some(access_token) => {
            match User::auth(access_token, &connection) {
                Some(user) => Response { success: true, user: Some(user.to_private()) },
                None => Response { success: false, user: None }
            }
        },
        None => Response { success: false, user: None }
    }
}
