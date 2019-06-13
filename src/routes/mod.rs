use rocket::Request;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    success: bool,
    data: T
}

pub mod access_tokens;
pub mod catchers;
pub mod users;
