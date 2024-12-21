use rocket::{Build, Rocket, Route};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::controller::constellation;
use crate::entity::constellation::*;
use crate::entity::star::Star;
use crate::repository::{constellations, stars};

#[get("/")]
fn list() -> Json<Vec<Constellation>> {
    Json(constellations::list_all())
}

#[get("/<id>")]
fn find(id: i32) -> Option<Json<Constellation>> {
    constellations::single(id).map(|s| Json(s))
}

#[get("/<id>/stars")]
fn get_stars(id: i32) -> Option<Json<Vec<Star>>> {
    constellations::stars(id).map(|s| Json(s))
}

#[post("/<id>/add_star", data = "<data>")]
fn add_star(id: i32, data: Json<i32>) -> Status {
    stars::add_to_constellation(data.into_inner(), id)
}

#[put("/", data = "<data>")]
fn update(data: Json<Constellation>) -> Status {
    if constellations::update(&data.into_inner()) {
        Status::Ok
    } else {
        Status::NotFound
    }
}

#[post("/", data = "<data>")]
fn insert(data: Json<NewConstellation>) -> Status {
    constellations::insert(data.into_inner());
    Status::Created
}

#[delete("/<id>")]
fn remove(id: i32) -> Status {
    if constellations::delete(id) {
        Status::NoContent
    } else {
        Status::NotFound
    }
}

pub fn routes() -> Vec<Route> {
    routes![list, get_stars, find, update, insert, remove, add_star]
}