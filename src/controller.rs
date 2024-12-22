use rocket::{Build, Rocket};
use rocket::fs::{relative, FileServer};

mod star;
mod constellation;
mod auth;

pub fn mount_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", FileServer::from(relative!("public")))
        .mount("/api/star", star::routes())
        .mount("/api/constellation", constellation::routes())
        .mount("/auth", auth::routes())
}