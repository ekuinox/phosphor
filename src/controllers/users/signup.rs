use crate::models::user::User;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSignupRequest {
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
pub struct UserSignupResponse {
    success: bool,
    user: Option<UserShow>,
    error_message: Option<String>
}

trait UserSignupResponseNew<T> {
    fn new(c: T) -> UserSignupResponse;
}

impl UserSignupResponseNew<User> for UserSignupResponse {
    fn new(user: User) -> UserSignupResponse {
        UserSignupResponse { success: true, user: Some(UserShow::new(user.username, user.id.unwrap(), user.email)), error_message: None }
    }
}

impl UserSignupResponseNew<String> for UserSignupResponse {
    fn new(error_message: String) -> UserSignupResponse {
        UserSignupResponse { success: false, user: None, error_message: Some(error_message) }
    }
}

pub fn signup(request: &UserSignupRequest, connection: &SqliteConnection) -> UserSignupResponse {
    if !request.username.is_ascii() {
        return UserSignupResponse { success: false, user: None, error_message: Some("username must be ascii".to_string()) };
    }

    match User::create(User::new(request.username.clone(),  request.email.clone(), request.password.clone()), &connection) {
        Some(user) => UserSignupResponse::new(user),
        None => UserSignupResponse::new("".to_string())
    }
}
