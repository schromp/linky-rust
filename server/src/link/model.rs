use diesel::prelude::*;

use crate::models;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_link(shortlink: String, conn: PgConnection) {


}