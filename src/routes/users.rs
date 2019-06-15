use rocket_contrib::json::Json;
use crate::controllers::users::signup;
use crate::controllers::users::login;
use crate::db;

// アカウントを生成
#[post("/users/signup", data = "<request>")]
pub fn signup(request: Json<signup::Request>, connection: db::Connection) -> Json<signup::Response> {
    Json(signup::signup(&request, &connection))
}

// ログイン
#[post("/users/login", data = "<request>")]
pub fn login(request: Json<login::Request>, connection: db::Connection) -> Json<login::Response> {
    Json(login::login(&request, &connection))
}
