#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;
mod auth;
mod models;
mod routes;
mod pool;

use rocket::{self, routes};
use self::routes::*;
use self::pool::DbConn;

fn main() {
    std::env::var("ADMIN_USERNAME").expect("ADMIN username must be set");
    std::env::var("ADMIN_PASSWORD").expect("ADMIN password must be set");
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/burgers", routes![
            get_burgers,
            create_burger,
            get_burger_by_id,
            get_random_burger])
        .launch();
}
