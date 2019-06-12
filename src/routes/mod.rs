use rocket::Request;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("404 {}", req)
}

pub mod users;
