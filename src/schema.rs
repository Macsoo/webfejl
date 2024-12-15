diesel::table! {
    star (id) {
        id -> Serial,
        right_ascension -> Float8,
        declination -> Float8,
        light_years -> Float4,
        magnitude -> Float4,
        bv_value -> Float4,
    }
}
