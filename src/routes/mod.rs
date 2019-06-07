use rocket::Request;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

// アカウントを生成
#[post("/account/signup", data = "<account>")]
pub fn acount_signup(account: String) -> String {
    format!("account/signup {:?}", account)
}

#[post("/article/create", data = "<article>")]
pub fn article_create(article: String) -> String {
    format!("/article/create {:?}", article)
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("404 {}", req)
}
