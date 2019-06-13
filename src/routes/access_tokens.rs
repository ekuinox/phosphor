use rocket_contrib::json::Json;
use crate::controllers::access_tokens::{create};
use crate::db;

// ログイン
#[post("/access_tokens/create", data = "<request>")]
pub fn create(request: Json<create::AccessTokenCreateRequest>, connection: db::Connection) -> Json<create::AccessTokenCreateResponse> {
    Json(create::create(&request, &connection))
}
