use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, article::Article, article::Accessible};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String,
    permalink: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
    article: Option<Article>
}

impl Response {
    pub fn new(success: bool, article: Option<Article>) -> Response {
        Response { success: success, article: article }
    }
}

pub fn show(request: &Request, connection: &SqliteConnection) -> Response {
    match AccessToken::auth(&request.token, &connection) {
        Some(access_token) => {
            match Article::get_by_permalink(request.permalink.clone(), &connection) {
                Some(article) => Response::new(true, Some(article)),
                None => Response::new(false, None)
            }
        },
        None => Response::new(false, None)
    }
}
