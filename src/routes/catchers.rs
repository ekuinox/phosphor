use rocket::Request;
use rocket_contrib::json::Json;
use crate::controllers::{ResponseBase, Error, Fail };

pub type ErrorResponse = ResponseBase<()>;

#[catch(400)]
pub fn bad_request(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Bad request".to_string(), 400)))
}

#[catch(404)]
pub fn not_found(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Not found".to_string(), 404)))
}

#[catch(422)]
pub fn unprocessable_entity(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Unprocessable entity".to_string(), 422)))
}

#[catch(500)]
pub fn internal_error(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse::fail(Error::new("Internal error".to_string(), 500)))
}
