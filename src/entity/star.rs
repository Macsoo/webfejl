use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
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