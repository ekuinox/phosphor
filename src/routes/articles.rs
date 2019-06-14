use rocket_contrib::json::Json;
use crate::controllers::articles::{create};
use crate::db;

// 記事の作成
#[post("/articles/create", data = "<request>")]
pub fn create(request: Json<create::Request>, connection: db::Connection) -> Json<create::Response> {
    Json(create::create(&request, &connection))
}
