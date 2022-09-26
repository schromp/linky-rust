mod config_app;
mod config_db;
mod db;
mod errors;
mod link;
mod routes;

use crate::config_app::config_app;

use ::config::Config;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use config_db::MyConfig;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: MyConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    //Init tne db
    //TODO change unwrap to handle errors and retry connecting
    if !std::env::var("NO_NEW_SETUP").is_ok() {
        let client = pool.get().await.unwrap();

        let stmt = include_str!("../sql/init_db.sql");
        client.batch_execute(&stmt).await.unwrap();
    }

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(config_app)
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr);
    server.await
}
