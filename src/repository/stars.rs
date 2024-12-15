use crate::entity::star::Star;
use crate::repository::*;
use diesel::prelude::*;
use crate::schema::star::dsl::star;

pub fn list_all() -> Vec<Star> {
    let connection = &mut establish_connection();
    star
        .select(Star::as_select())
        .load(connection)
        .expect("Error loading stars")
}