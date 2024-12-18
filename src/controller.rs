use rocket::{Build, Rocket};

mod star;
mod constellation;

pub fn mount_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/star", star::routes())
        .mount("/constellation", constellation::routes())
}