use rocket::Request;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    success: bool,
    data: T
}

pub mod access_tokens;
pub mod catchers;
pub mod users;
