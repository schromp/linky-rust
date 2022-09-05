use serde::{Deserialize, Serialize};
pub mod routes;

#[derive(Deserialize, Serialize)]
pub struct Link {
    shortlink: String,
    longlink: String,
}

