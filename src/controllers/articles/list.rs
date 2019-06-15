use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, article::Article, article::Accessible, article::PermalinksListData};

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
    match User::auth(&request.token, &connection) {
        Some(user) => {
            match Article::get_list_by_user_id(user.id.unwrap(), 500, &connection) {
                Some(permalinks) => Response::new(true, Some(permalinks)),
                None => Response::new(false, None)
            }
        },
        None => Response::new(false, None)
    }
}
