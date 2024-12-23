use diesel::insert_into;
use crate::entity::star::*;
use crate::repository::*;
use diesel::prelude::*;
use diesel::result::*;
use rocket::http::Status;
use crate::schema::star::dsl::*;

pub fn list_all() -> Vec<Star> {
    let connection = &mut establish_connection();
    star
        .select(Star::as_select())
        .load(connection)
        .expect("Error loading stars")
}

pub fn single(star_id: i32) -> Option<Star> {
    let connection = &mut establish_connection();
    star
        .filter(id.eq(star_id))
        .get_result(connection)
        .optional()
        .expect("Error loading star")
}

pub fn insert(new_star: NewStar) {
    let connection = &mut establish_connection();
    insert_into(star)
        .values(new_star)
        .execute(connection)
        .expect("Error adding star");
}

pub fn delete(star_id: i32) -> bool {
    let connection = &mut establish_connection();
    1 == diesel::delete(star.filter(id.eq(star_id)))
        .execute(connection)
        .expect("Error deleting star")
}

pub fn update(star_entity: &Star) -> bool {
    let connection = &mut establish_connection();
    1 == diesel::update(star.filter(id.eq(star_entity.id)))
        .set(star_entity)
        .execute(connection)
        .expect("Error updating star")
}

pub fn add_to_constellation(star_id: i32, const_id: i32) -> Status {
    let connection = &mut establish_connection();
    let update_result = diesel::update(star.filter(id.eq(star_id)))
        .set(constellation_id.eq(const_id))
        .execute(connection);
    match update_result {
        Ok(1) => Status::Ok,
        Ok(0) => Status::NotFound,
        Err(Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, _)) => Status::NotAcceptable,
        _ => Status::InternalServerError,
    }
}