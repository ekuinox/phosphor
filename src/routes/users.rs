use rocket_contrib::json::Json;
use crate::controllers::users::signup;
use crate::db;

// アカウントを生成
#[post("/users/signup", data = "<request>")]
pub fn signup(request: Json<signup::UserSignupRequest>, connection: db::Connection) -> Json<signup::UserSignupResponse> {
    Json(signup::signup(&request, &connection))
}
