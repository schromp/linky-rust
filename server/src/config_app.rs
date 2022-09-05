use actix_web::{web::{self}, HttpResponse};

use crate::link;

pub fn config_app(cfg: &mut web::ServiceConfig) {


    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("base request")}))
    )


    .service(
        web::resource("/createLink")
            .route(web::get().to(link::routes::create_link))
    )


    .service(
        web::resource("/getAll")
            .route(web::get().to(link::routes::get_all))   
    )


    .service(
        web::resource("/{somelink}")
            .route(web::get().to(link::routes::get_link))
    );
}