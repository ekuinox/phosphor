use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, article::Article, article::Accessible, article::PermalinksListData};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
    data: Option<Vec<PermalinksListData>>
}

impl Response {
    pub fn new(success: bool, data: Option<Vec<PermalinksListData>>) -> Response {
        Response { success: success, data: data }
    }
}

pub fn list(request: &Request, connection: &SqliteConnection) -> Response {
    match AccessToken::from_string(&request.token, &connection) {
        Some(access_token) => {
            match Article::get_list_by_user_id(access_token.user_id, 500, &connection) {
                Some(permalinks) => Response::new(true, Some(permalinks)),
                None => Response::new(false, None)
            }
        },
        None => Response::new(false, None)
    }
}
