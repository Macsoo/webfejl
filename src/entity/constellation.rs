use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::constellation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Constellation {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::constellation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewConstellation {
    pub name: String,
}