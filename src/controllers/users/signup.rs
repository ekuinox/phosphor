use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use crate::controllers::ResponseBase;
use crate::models::user::{User, PrivateUserData, ToPrivate};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    username: String,
    password: String,
    email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    user: PrivateUserData
}

impl Data {
    pub fn new(user: PrivateUserData) -> Data {
        Data { user: user }
    }
}

pub type Response = ResponseBase<Data, ()>;

pub fn signup(request: &Request, connection: &SqliteConnection) -> Response {
    if !request.username.is_ascii() {
        return Response::fail(());
    }

    match User::create(User::new(request.username.clone(),  request.email.clone(), request.password.clone()), &connection) {
        Some(user) => Response::success(Data::new(user.to_private())),
        None => Response::fail(())
    }
}
