mod repository;
mod schema;
mod entity;
mod controller;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    controller::mount_routes(rocket::build())
}