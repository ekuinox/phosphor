use rocket::Request;
use rocket_contrib::json::Json;
use crate::controllers::ResponseBase;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message: message }
    }
}

pub type ErrorResponse = ResponseBase<Error>;

#[catch(400)]
pub fn bad_request(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Bad request".to_string())))
}

#[catch(404)]
pub fn not_found(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Not found".to_string())))
}

#[catch(422)]
pub fn unprocessable_entity(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Unprocessable entity".to_string())))
}

#[catch(500)]
pub fn internal_error(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Internal error".to_string())))
}
