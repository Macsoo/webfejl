mod repository;
mod schema;
mod entity;

#[macro_use] extern crate rocket;

use itertools::Itertools;
use crate::repository::*;

#[get("/")]
fn index() -> String {
    stars::list_all()
        .into_iter()
        .map(|s| format!("{:#?}", s))
        .join("\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}