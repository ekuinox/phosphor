use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::models::{access_token::*, user::*};
use crate::controllers::{ResponseBase, Error as ErrorStruct, ToError, Fail};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    InternalError,
    BadCredentials
}

impl ToError for Error {
    fn to_error(&self) -> ErrorStruct {
        match self {
            InternalError => ErrorStruct::new(String::from("InternalError"), 1),
            BasicCredentials => ErrorStruct::new(String::from("BadCredentials"), 2)
        }
    }
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

pub type Response = ResponseBase<Data>;

pub fn touch(request: &Request, connection: &SqliteConnection) -> Response {
    match User::auth(&request.token, &connection) {
        Ok(user) => Response::success(Data::new(user.to_private())),
        Err(_) => Response::fail(Error::BadCredentials)
    }
}
