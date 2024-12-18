diesel::table! {
    star (id) {
        id -> Serial,
        right_ascension -> Float8,
        declination -> Float8,
        light_years -> Float4,
        magnitude -> Float4,
        bv_value -> Float4,
        constellation_id -> Nullable<Int4>,
    }
}

diesel::table! {
    constellation (id) {
        id -> Serial,
        name -> VarChar,
    }
}

diesel::joinable!(star -> constellation (constellation_id));

diesel::allow_tables_to_appear_in_same_query!(star, constellation);
