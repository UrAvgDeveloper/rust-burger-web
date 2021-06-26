use serde::{Serialize, Deserialize};
use rocket::request::{FromForm};
use crate::schema::burger;

#[derive(Queryable, Serialize)]
pub struct Burger {
    id: i32,
    name: String,
    image_url: Option<String>,
    description: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[table_name="burger"]
pub struct NewBurger {
    name: String,
    image_url: Option<String>,
    description: Option<String>,
}

#[derive(FromForm)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}