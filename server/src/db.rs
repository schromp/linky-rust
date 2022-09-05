use diesel::pg::PgConnection;
use diesel::{ r2d2};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>>  {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATBASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create Pool");
    pool
    
    // PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}