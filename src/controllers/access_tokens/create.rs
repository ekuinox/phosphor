use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::AccessToken, user::User, user::Authenticate, user::BasicCredentials};
use crate::controllers::ResponseBase;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    InternalError,
    BadCredentials
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    token: String
}

impl Data {
    pub fn new(token: String) -> Data {
        Data {
            token: token
        }
    }
}

pub type Response = ResponseBase<Data, Error>;

pub fn create(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(BasicCredentials::new(request.username.clone(), request.password.clone()), &connection) {
        Some(user) => {
            match AccessToken::new(user.id.unwrap()).create(&connection) {
                Some(access_token) => Response::success(Data::new(access_token.token.unwrap())),
                None => Response::fail(Error::InternalError)
            }
        },
        None => Response::fail(Error::BadCredentials)
    }
}
