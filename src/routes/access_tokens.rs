use rocket_contrib::json::Json;
use crate::controllers::access_tokens::{create, is_valid};
use crate::db;

// ログイン
#[post("/access_tokens/create", data = "<request>")]
pub fn create(request: Json<create::AccessTokenCreateRequest>, connection: db::Connection) -> Json<create::AccessTokenCreateResponse> {
    Json(create::create(&request, &connection))
}

// トークンの有効性確認
#[post("/access_tokens/is_valid", data = "<request>")]
pub fn is_valid(request: Json<is_valid::Request>, connection: db::Connection) -> Json<is_valid::Response> {
    Json(is_valid::is_valid(&request, &connection))
}
