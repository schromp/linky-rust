mod config_app;
mod link;
mod config_db;
mod routes;
mod errors;
mod db;


use crate::config_app::config_app;

use actix_web::{HttpServer, App, web};
use config_db::MyConfig;
use dotenv::dotenv;
use ::config::Config;
use tokio_postgres::NoTls;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config_ = Config::builder().add_source(::config::Environment::default()).build().unwrap();

    let config: MyConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();


    //Init tne db

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