mod diesel_test;
mod models;
mod schema;

#[macro_use] extern crate rocket;

use crate::diesel_test::establish_connection;
use diesel::prelude::*;
use crate::models::Post;

#[get("/")]
fn index() -> String {
    use self::schema::posts::dsl::*;
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");
    results.iter()
        .map(|p| format!("{:#?}", p))
        .collect::<Vec<_>>()
        .join("\n")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}