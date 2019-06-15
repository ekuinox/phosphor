use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use yyid::yyid_string;
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, article::Article, article::Accessible};
use crate::controllers::{ResponseBase, Fail};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String,
    title: String,
    body: String,
    permalink: Option<String>,
    accessible: Option<Accessible>
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

pub fn create(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(&request.token, &connection) {
        Some(user) => {
            match Article::new_with_now(
                user.id.unwrap(),
                request.title.clone(),
                request.body.clone(),
                match &request.permalink {
                    Some(permalink) => permalink.clone(),
                    None => yyid_string()
                },
                request.accessible.unwrap_or(Accessible::Public)
                ).insert(connection) {
                Some(article) => Response::success(Data::new(article)),
                None => Response::fail(())
            }
        },
        None => Response::fail(())
    }
}
