use actix_web::{web::{self}, HttpResponse};

use crate::{ routes};

pub fn config_app(cfg: &mut web::ServiceConfig) {


    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("base request")}))
    )


    .service(
        web::resource("/createLink")
            .route(web::post().to(routes::create_link))
    )


    .service(
        web::resource("/getAll")
            .route(web::get().to(routes::get_all))   
    )


    .service(
        web::resource("/{somelink}")
            .route(web::get().to(routes::get_link))
    );
}