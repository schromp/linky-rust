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
use std::{thread, time};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();


    let config: MyConfig = config_.try_deserialize().unwrap();


    let pool = config.pg.create_pool(None, NoTls).unwrap();

    //get a client from the pool and try out a connection. if failes retry

    let mut retries = 5;
    while retries > 0 {

        let client = pool.get().await;
        match client {
            Ok(c) => {
                println!("Database connection successfull");
                retries = 0;

                //if connected check for no new setup
                if !std::env::var("NO_NEW_SETUP").is_ok() {
            
                    let stmt = include_str!("../sql/init_db.sql");
                    c.batch_execute(&stmt).await.unwrap();
                }
            },
            Err(e) => {
                println!("Database connection error. Retrying in 5 seconds. {retries}/5");
                eprintln!("{}", e);
                thread::sleep(time::Duration::from_secs(5));
                retries = retries - 1; 
            }
        };

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
