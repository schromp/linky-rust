// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        shortlink -> Varchar,
        longlink -> Varchar,
    }
}
