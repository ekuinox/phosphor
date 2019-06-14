#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

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
mod macros;

use routes::*;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![users::signup, users::login, access_tokens::create, access_tokens::touch, articles::create])
        .register(catchers![catchers::bad_request, catchers::unprocessable_entity, catchers::not_found, catchers::internal_error])
        .launch();
}
