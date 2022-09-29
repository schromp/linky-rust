use deadpool_postgres::Client;
use rand::{distributions::Alphanumeric, Rng};
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{errors::MyError, link::Link};

pub async fn create_link(client: &Client, link_info: Link) -> Result<Link, MyError> {
    let _stmt = include_str!("../sql/create_link.sql");
    let _stmt = _stmt.replace("$table_fields", &Link::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    //random string logic
    if link_info.shortlink.is_empty() {
        let mut tested_shortstring = String::new();
        for _ in 1..10 {
            let s: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();
            let req = get_link(client, &s).await;

            match req {
                Ok(_) => continue,
                Err(_) => {
                    tested_shortstring = s;
                    break;
                }
            }
        }

        let res = client
            .query(&stmt, &[&tested_shortstring, &link_info.longlink])
            .await?
            .iter()
            .map(|row| Link::from_row_ref(row).unwrap())
            .collect::<Vec<Link>>()
            .pop()
            .ok_or(MyError::NotFound);

        res
    } else {
        client
            .query(&stmt, &[&link_info.shortlink, &link_info.longlink])
            .await?
            .iter()
            .map(|row| Link::from_row_ref(row).unwrap())
            .collect::<Vec<Link>>()
            .pop()
            .ok_or(MyError::NotFound)
    }
}

///
/// No advanced error handling because actix returns server error if something is not working
/// -> Not worth the hustle
///
///
pub async fn get_all(client: &Client) -> Result<Vec<Link>, MyError> {
    let rows = client.query("SELECT * FROM linky.link", &[]).await?;

    let rows2 = rows
        .iter()
        .map(|row| Link::from_row_ref(row).unwrap())
        .collect::<Vec<Link>>();
    Ok(rows2)
}

pub async fn get_link(client: &Client, to_get: &str) -> Result<Link, MyError> {
    let _stmt = include_str!("../sql/get_link.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    client
        .query(&stmt, &[&to_get])
        .await?
        .iter()
        .map(|row| Link::from_row_ref(row).unwrap())
        .collect::<Vec<Link>>()
        .pop()
        .ok_or(MyError::NotFound)
}
