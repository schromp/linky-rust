use diesel::prelude::*;

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub shortlink: String,
    pub longlink: String,
}