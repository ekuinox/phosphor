use rocket_contrib::json::Json;
use crate::controllers::access_tokens::{create, touch};
use crate::db;

// ログイン
#[post("/access_tokens/create", data = "<request>")]
pub fn create(request: Json<create::AccessTokenCreateRequest>, connection: db::Connection) -> Json<create::AccessTokenCreateResponse> {
    Json(create::create(&request, &connection))
}

// トークンの更新（有効性確認）
#[post("/access_tokens/touch", data = "<request>")]
pub fn touch(request: Json<touch::Request>, connection: db::Connection) -> Json<touch::Response> {
    Json(touch::touch(&request, &connection))
}
