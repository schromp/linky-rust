use actix_web::{HttpResponse, Error, web};
use deadpool_postgres::{Pool, Client};
use crate::{db, errors::MyError, link::Link};

//TNOT TESTED!
pub async fn get_link(toget: web::Path<String>,  db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

    let sl = toget.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let linkk = db::get_link(&client, &sl).await?;

    Ok(HttpResponse::Ok().json(linkk.shortlink))

}

pub async fn create_link(json: web::Json<Link>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let link_info: Link = json.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_link = db::create_link(&client, link_info).await?;

    Ok(HttpResponse::Ok().json(new_link))
}

pub async fn get_all(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let linkk = db::get_all(&client).await?;
    Ok(HttpResponse::Ok().json(linkk))
}