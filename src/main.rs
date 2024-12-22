mod repository;
mod schema;
mod entity;
mod controller;
mod claims;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().unwrap();
    controller::mount_routes(rocket::build())
}