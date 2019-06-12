#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
#[macro_use] extern crate argon2;

mod routes;
mod schema;
mod db;
mod models;
mod controllers;

use routes::*;

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index, users::signup, users::login])
        .register(catchers![not_found])
        .launch();
}
