#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::Request;

mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, article_create, acount_signup])
        .register(catchers![not_found])
        .launch();
}
