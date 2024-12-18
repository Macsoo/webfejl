use diesel::insert_into;
use crate::entity::constellation::*;
use crate::repository::*;
use diesel::prelude::*;
use crate::entity::star::Star;
use crate::schema::constellation::dsl::*;
use crate::schema::star::dsl::{star, constellation_id};

pub fn list_all() -> Vec<Constellation> {
    let connection = &mut establish_connection();
    constellation
        .select(Constellation::as_select())
        .load(connection)
        .expect("Error loading constellations")
}

pub fn single(const_id: i32) -> Option<Constellation> {
    let connection = &mut establish_connection();
    constellation
        .filter(id.eq(const_id))
        .get_result(connection)
        .optional()
        .expect("Error loading constellation")
}

pub fn insert(new_constellation: NewConstellation) {
    let connection = &mut establish_connection();
    insert_into(constellation)
        .values(new_constellation)
        .execute(connection)
        .expect("Error adding constellation");
}

pub fn delete(const_id: i32) -> bool {
    let connection = &mut establish_connection();
    1 == diesel::delete(constellation.filter(id.eq(const_id)))
        .execute(connection)
        .expect("Error deleting constellation")
}

pub fn update(constellation_entity: &Constellation) -> bool {
    let connection = &mut establish_connection();
    1 == diesel::update(constellation.filter(id.eq(constellation_entity.id)))
        .set(constellation_entity)
        .execute(connection)
        .expect("Error updating constellation")
}

pub fn stars(const_id: i32) -> Option<Vec<Star>> {
    let connection = &mut establish_connection();
    if 1 != constellation.find(const_id)
        .execute(connection)
        .expect("Error loading constellation") {
        return None;
    }
    let res = star.filter(constellation_id.eq(const_id))
        .load(connection)
        .expect("Error getting stars");
    Some(res)
}