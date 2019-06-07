#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;

mod routes;
mod schema;
mod db;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, article_create, acount_signup])
        .register(catchers![not_found])
        .launch();
}
