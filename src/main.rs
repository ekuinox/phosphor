#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate rocket_contrib;
extern crate serde_json;
extern crate argon2;
extern crate yyid;

mod routes;
mod schema;
mod db;
mod models;
mod controllers;

use routes::*;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index, users::signup, users::login, access_tokens::create, access_tokens::is_valid])
        .register(catchers![not_found])
        .launch();
}
