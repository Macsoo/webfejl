use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::star)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Star {
    pub id: i32,
    pub right_ascension: f64,
    pub declination: f64,
    pub light_years: f32,
    pub magnitude: f32,
    pub bv_value: f32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::star)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStar {
    pub right_ascension: f64,
    pub declination: f64,
    pub light_years: f32,
    pub magnitude: f32,
    pub bv_value: f32,
}