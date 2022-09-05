use actix_web::{HttpResponse, Error, web};

use super::Link;

pub async fn get_link(_somelink: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("this is a link"))
}

pub async fn create_link(json: web::Json<Link>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(json.longlink.to_string()))
}

pub async fn get_all() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("here come all links"))
}