mod config_app;
mod link;
mod db;

use actix_web::{HttpServer, App, web};
use crate::config_app::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //TODO Connection Pool to pass into HTTP Server
    let pool = db::establish_connection();
    println!("Connected to DB.");
    

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config_app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}