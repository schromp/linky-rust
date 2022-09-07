use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, link::Link};

pub async fn create_link(client: &Client, link_info: Link) -> Result<Link, MyError> {
    let _stmt = include_str!("../sql/create_link.sql");
    let _stmt = _stmt.replace("$table_fields", &Link::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
    .query(
        &stmt,
        &[
            &link_info.shortlink,
            &link_info.longlink,
        ],
    )
    .await?
    .iter()
    .map(|row| Link::from_row_ref(row).unwrap())
    .collect::<Vec<Link>>()
    .pop()
    .ok_or(MyError::NotFound) // more applicable for SELECTs
}

//this only selects the last created object in the database
//have to go through everything after await
pub async fn get_all(client: &Client) -> Result<Link, MyError> {
    client.query("SELECT * FROM linky.link", &[])
        .await?
        .iter()
        .map(|row| Link::from_row_ref(row).unwrap())
        .collect::<Vec<Link>>()
        .pop()
        .ok_or(MyError::NotFound)
}