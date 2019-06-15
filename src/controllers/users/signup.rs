use crate::models::user::User;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    username: String,
    password: String,
    email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserShow {
    username: String,
    id: i32,
    email: String
}

impl UserShow {
    pub fn new(username: String, id: i32, email: String) -> UserShow {
        UserShow { username: username, id: id, email: email }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    success: bool,
    user: Option<UserShow>,
    error_message: Option<String>
}

trait ResponseNew<T> {
    fn new(c: T) -> Response;
}

impl ResponseNew<User> for Response {
    fn new(user: User) -> Response {
        Response { success: true, user: Some(UserShow::new(user.username, user.id.unwrap(), user.email)), error_message: None }
    }
}

impl ResponseNew<String> for Response {
    fn new(error_message: String) -> Response {
        Response { success: false, user: None, error_message: Some(error_message) }
    }
}

pub fn signup(request: &Request, connection: &SqliteConnection) -> Response {
    if !request.username.is_ascii() {
        return Response { success: false, user: None, error_message: Some("username must be ascii".to_string()) };
    }

    match User::create(User::new(request.username.clone(),  request.email.clone(), request.password.clone()), &connection) {
        Some(user) => Response::new(user),
        None => Response::new("".to_string())
    }
}
