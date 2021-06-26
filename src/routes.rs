use crate::schema::burger;
use rocket::{self, get, post};
use rocket::request::{Form};
use rocket_contrib::json::Json;
use diesel::prelude::*;
use crate::auth::Admin;
use crate::pool::DbConn;
use crate::models::*;

#[get("/?<burger_name>&<pagination..>")]
pub fn get_burgers(conn: DbConn, burger_name: Option<String>, pagination: Form<PaginationParams>) -> QueryResult<Json<Vec<Burger>>> {
    let limit = match pagination.per_page {
        None => 30,
        Some(per_page) => per_page,
    };

    let offset = match pagination.page {
        None => 0,
        Some(page) => (page - 1) * limit,
    };

    let burger_name = match burger_name {
        None => String::from(""),
        Some(burger_name) => burger_name,
    };

    burger::table
        .filter(burger::columns::name.ilike(format!("%{}%", burger_name)))
        .order(burger::columns::id.asc())
        .limit(limit)
        .offset(offset)
        .load::<Burger>(&*conn)
        .map(|xs| Json(xs))
}

#[post("/", data = "<new_burger>")]
pub fn create_burger(conn: DbConn, _admin: Admin, new_burger: Json<NewBurger>) -> QueryResult<Json<Burger>> {
    diesel::insert_into(burger::table)
        .values(&*new_burger)
        .get_result(&*conn)
        .map(|x| Json(x))
}

#[get("/<id>")]
pub fn get_burger_by_id(conn: DbConn, id: i32) -> QueryResult<Json<Burger>> {
    burger::table
        .find(id)
        .get_result(&*conn)
        .map(|x| Json(x))
}

#[get("/random")]
pub fn get_random_burger(conn: DbConn) -> QueryResult<Json<Burger>> {
    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");
    burger::table
        .order(RANDOM)
        .limit(1)
        .get_result(&*conn)
        .map(|x| Json(x))
}