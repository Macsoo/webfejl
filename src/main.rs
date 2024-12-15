mod repository;
mod schema;
mod entity;

#[macro_use] extern crate rocket;

use itertools::Itertools;
use rocket::http::Status;
use rocket::Response;
use rocket::serde::json::Json;
use crate::entity::star::{NewStar, Star};
use crate::repository::*;

#[get("/stars")]
fn star_list() -> Json<Vec<Star>> {
    Json(stars::list_all())
}

#[get("/stars/<id>")]
fn star_single(id: i32) -> Option<Json<Star>> {
    stars::single(id).map(|s| Json(s))
}

#[post("/stars", data = "<data>")]
fn star_insert(data: Json<NewStar>) -> Status {
    stars::insert(data.into_inner());
    Status::Ok
}

#[delete("/stars/<id>")]
fn star_delete(id: i32) -> Status {
    if stars::delete(id) {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        star_list,
        star_single,
        star_insert,
        star_delete,
    ])
}