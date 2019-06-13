use rocket::Request;
use rocket_contrib::json::Json;
use crate::controllers::ResponseBase;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    message: String
}

pub type ErrorResponse = ResponseBase<Error>;

#[catch(400)]
pub fn bad_request(request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error { message: "Bad request".to_string() }))
}

#[catch(404)]
pub fn not_found(request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error { message: "Not found".to_string() }))
}

#[catch(422)]
pub fn unprocessable_entity(request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error { message: "Unprocessable entity".to_string() }))
}

#[catch(500)]
pub fn internal_error(request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error { message: "Internal error".to_string() }))
}
