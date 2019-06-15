use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::*, user::*};
use crate::controllers::ResponseBase;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    InternalError,
    BadCredentials
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    user: PrivateUserData
}

impl Data {
    pub fn new(user: PrivateUserData) -> Data {
        Data {
            user: user
        }
    }
}

pub type Response = ResponseBase<Data, Error>;

pub fn touch(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(&request.token, &connection) {
        Some(user) => Response::success(Data::new(user.to_private())),
        None => Response::fail(Error::BadCredentials)
    }
}
