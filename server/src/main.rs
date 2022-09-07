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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config_ = Config::builder().add_source(::config::Environment::default()).build().unwrap();

    let config: MyConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config_app)
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr);
    server.await
}