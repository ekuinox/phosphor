use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, article::Article, article::Accessible};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String,
    title: String,
    body: String,
    permalink: String,
    accessible: Option<Accessible>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
}

impl Response {
    pub fn new(success: bool) -> Response {
        Response { success: success }
    }
}

pub fn create(request: &Request, connection: &SqliteConnection) -> Response {
    match AccessToken::auth(&request.token, &connection) {
        Some(access_token) => {
            match Article::new_with_now(
                access_token.user_id.clone(),
                request.title.clone(),
                request.body.clone(),
                request.permalink.clone(),
                request.accessible.unwrap_or(Accessible::Public)
                ).insert(connection) {
                Some(article) => Response::new(true),
                None => Response::new(false)
            }
        },
        None => Response::new(false)
    }
}
