use rocket::{Build, Rocket, Route};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::entity::star::{NewStar, Star};
use crate::repository::stars;

#[get("/")]
fn list() -> Json<Vec<Star>> {
    Json(stars::list_all())
}

#[get("/<id>")]
fn find(id: i32) -> Option<Json<Star>> {
    stars::single(id).map(|s| Json(s))
}

#[put("/", data = "<data>")]
fn update(data: Json<Star>) -> Status {
    if stars::update(&data.into_inner()) {
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[post("/", data = "<data>")]
fn insert(data: Json<NewStar>) -> Status {
    stars::insert(data.into_inner());
    Status::Created
}

#[delete("/<id>")]
fn remove(id: i32) -> Status {
    if stars::delete(id) {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

pub fn routes() -> Vec<Route> {
    routes![list, find, update, insert, remove]
}