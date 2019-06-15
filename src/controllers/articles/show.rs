use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, article::Article, article::Accessible};
use crate::controllers::{ResponseBase, Fail};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String,
    permalink: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    article: Article
}

impl Data {
    pub fn new(article: Article) -> Data {
        Data { article: article }
    }
}

pub type Response = ResponseBase<Data>;

pub fn show(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(&request.token, &connection) {
        Ok(user) => {
            match Article::get_by_permalink(request.permalink.clone(), &connection) {
                Some(article) => Response::success(Data::new(article)),
                None => Response::fail(())
            }
        },
        Err(_) => Response::fail(())
    }
}
