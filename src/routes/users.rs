use rocket_contrib::json::Json;
use crate::controllers::users::signup;
use crate::controllers::users::login;
use crate::db;

// アカウントを生成
#[post("/users/signup", data = "<request>")]
pub fn signup(request: Json<signup::UserSignupRequest>, connection: db::Connection) -> Json<signup::UserSignupResponse> {
    Json(signup::signup(&request, &connection))
}

// ログイン
#[post("/users/login", data = "<request>")]
pub fn login(request: Json<login::UserLoginRequest>, connection: db::Connection) -> Json<login::UserLoginResponse> {
    Json(login::login(&request, &connection))
}
