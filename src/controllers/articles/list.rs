use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, article::Article, article::Accessible, article::PermalinksListData};
use crate::controllers::ResponseBase;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    permalinks: Vec<PermalinksListData>
}

impl Data {
    pub fn new(permalinks: Vec<PermalinksListData>) -> Data {
        Data { permalinks: permalinks }
    }
}

pub type Response = ResponseBase<Data, ()>;

pub fn list(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(&request.token, &connection) {
        Some(user) => {
            match Article::get_list_by_user_id(user.id.unwrap(), 500, &connection) {
                Some(permalinks) => Response::success(Data::new(permalinks)),
                None => Response::fail(())
            }
        },
        None => Response::fail(())
    }
}
