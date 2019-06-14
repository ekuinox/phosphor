use rocket_contrib::json::Json;
use crate::controllers::articles::{create, list, show};
use crate::db;

// 記事の作成
#[post("/articles/create", data = "<request>")]
pub fn create(request: Json<create::Request>, connection: db::Connection) -> Json<create::Response> {
    Json(create::create(&request, &connection))
}

// 記事のリスト
#[get("/articles/list", data = "<request>")]
pub fn list(request: Json<list::Request>, connection: db::Connection) -> Json<list::Response> {
    Json(list::list(&request, &connection))
}

// 記事の取得
#[get("/articles/show", data = "<request>")]
pub fn show(request: Json<show::Request>, connection: db::Connection) -> Json<show::Response> {
    Json(show::show(&request, &connection))
}
