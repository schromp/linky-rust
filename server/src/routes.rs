use actix_web::{HttpResponse, Error, web, http};
use deadpool_postgres::{Pool, Client};
use crate::{db, errors::MyError, link::Link};
use url::{Url, ParseError};


///
/// fetches the link from the database
/// redirects to the longlink
/// 
/// TODO check if link available in database and return error page if not
/// 
pub async fn get_link(somelink: web::Path<String>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    
    let client = db_pool.get().await.map_err(MyError::PoolError)?;

    let lookup = db::get_link(&client, &somelink).await?;
    
    Ok(HttpResponse::TemporaryRedirect().status(http::StatusCode::TEMPORARY_REDIRECT).insert_header(("location", lookup.longlink)).finish())

}

//TODO check link for formatting and return different error in case link already exists
pub async fn create_link(json: web::Json<Link>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let link_info: Link = json.into_inner();

    // if Url::parse(&link_info.longlink) == Err(_) {
    //     println!("help");
    //     return Ok(HttpResponse::BadRequest().finish());
    // } 
    // cant get this stupid shit to work.....

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_link = db::create_link(&client, link_info).await?;

    Ok(HttpResponse::Ok().json(new_link))
}

pub async fn get_all(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let linkk = db::get_all(&client).await?; //advanced error handling?
    Ok(HttpResponse::Ok().json(linkk))
}